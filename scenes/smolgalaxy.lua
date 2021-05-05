local unpack = table.unpack or _G.unpack
util.seed()

local SCALE = 0.3
local N_EACH = 25

local function randtable(n, a, b)
	local t = {}
	for i=1, n do
		t[i] = util.randab(a, b)
	end
	return t
end

local MATERIALS = {surfacetype.DIFFUSE, surfacetype.REFLECTIVE, surfacetype.STOP}
local function randommaterial()
	local emission = colorvec.new(unpack(randtable(4, 0, 1)))
	local reflection = colormat.new(unpack(randtable(16, 0, 1)))
	local surfacetype = util.choice(MATERIALS)

	return material.new(emission, reflection, surfacetype)
end

local ORIENTATIONS = {
	mat3.new(
		1, 0, 0,
		0, 1, 0,
		0, 0, 1
	),
	mat3.new(
		1, 0, 0,
		0, 0, 1,
		0, 1, 0
	),
	mat3.new(
		0, 1, 0,
		1, 0, 0,
		0, 0, 1
	),
	mat3.new(
		0, 1, 0,
		0, 0, 1,
		1, 0, 0
	),
	mat3.new(
		0, 0, 1,
		0, 1, 0,
		1, 0, 0
	),
	mat3.new(
		0, 0, 1,
		1, 0, 0,
		0, 1, 0
	)
}
local function randomorientation()
	return util.choice(ORIENTATIONS)
end

local function randomsphere()
	local pos = vec3.new(unpack(randtable(3, -2, 2)))
	local radius = util.randab(0, SCALE)

	return obj.sphere(pos, radius)
end

local function randomtorus()
	local pos = vec3.new(unpack(randtable(3, -2, 2)))
	local radius = util.randab(0, SCALE)
	local thickness = util.randab(radius/4, radius)
	local orientation = randomorientation()

	return util.transform(obj.torus(pos, radius, thickness), orientation)
end

local function randomcuboid()
	local pos = vec3.new(unpack(randtable(3, -2, 2)))
	local radius = util.randab(0, SCALE)

	return obj.cuboid(pos, vec3.new(radius, radius, radius))
end

local function randomcylinder()
	local pos = vec3.new(unpack(randtable(3, -2, 2)))
	local radius = util.randab(0, SCALE)
	local height = util.randab(0, SCALE)
	local orientation = randomorientation()

	return util.transform(
		obj.intersection(
			obj.cylinder(pos, radius),
			obj.cuboid(pos, vec3.new(radius, height, radius))
		),
		orientation
	)
end

local objects = {}
for i=1, N_EACH do table.insert(objects, randomsphere()) end
for i=1, N_EACH do table.insert(objects, randomtorus()) end
for i=1, N_EACH do table.insert(objects, randomcuboid()) end
for i=1, N_EACH do table.insert(objects, randomcylinder()) end
for i, object in ipairs(objects) do objects[i] = obj.withmaterial(object, randommaterial()) end
return util.union(objects)