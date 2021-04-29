local unpack = table.unpack or _G.unpack
math.randomseed(os.time())

local SCALE = 0.3
local N_EACH = 25

local function randab(a, b)
	return math.random() * (b-a) + a
end

local function randtable(n, a, b)
	local t = {}
	for i=1, n do
		t[i] = randab(a, b)
	end
	return t
end

local function choice(list)
	return list[math.random(1, #list)]
end

local MATERIALS = {surfacetype.DIFFUSE, surfacetype.REFLECTIVE, surfacetype.STOP}
local function randommaterial()
	local emission = colorvec.new(unpack(randtable(4, 0, 1)))
	local reflection = colormat.new(unpack(randtable(16, 0, 1)))
	local surfacetype = choice(MATERIALS)

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
	return choice(ORIENTATIONS)
end

local function randomsphere()
	local pos = vec3.new(unpack(randtable(3, -2, 2)))
	local radius = randab(0, SCALE)

	return obj.sphere(pos, radius)
end

local function randomtorus()
	local pos = vec3.new(unpack(randtable(3, -2, 2)))
	local radius = randab(0, SCALE)
	local thickness = randab(radius/4, radius)
	local orientation = randomorientation()

	return obj.affinetransform(obj.torus(pos, radius, thickness), orientation, vec3.O)
end

local function randomcuboid()
	local pos = vec3.new(unpack(randtable(3, -2, 2)))
	local radius = randab(0, SCALE)

	return obj.cuboid(pos, vec3.new(radius, radius, radius))
end

local function randomcylinder()
	local pos = vec3.new(unpack(randtable(3, -2, 2)))
	local radius = randab(0, SCALE)
	local height = randab(0, SCALE)
	local orientation = randomorientation()

	return obj.affinetransform(
		obj.intersection(
			obj.cylinder(pos, radius),
			obj.cuboid(pos, vec3.new(radius, height, radius))
		),
		orientation,
		vec3.O
	)
end

local function union(objs)
	local n = #objs
	if n == 1 then return objs[1] end

	local mid = n//2
	local left, right = {}, {}
	for i=1, mid do
		left[i] = objs[i]
	end
	for i=mid+1, n do
		right[i-mid] = objs[i]
	end
	return obj.union(union(left), union(right))
end

local objects = {}
for i=1, N_EACH do table.insert(objects, randomsphere()) end
for i=1, N_EACH do table.insert(objects, randomtorus()) end
for i=1, N_EACH do table.insert(objects, randomcuboid()) end
for i=1, N_EACH do table.insert(objects, randomcylinder()) end
for i, object in ipairs(objects) do objects[i] = obj.withmaterial(object, randommaterial()) end
return union(objects)