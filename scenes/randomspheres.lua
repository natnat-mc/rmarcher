local unpack = table.unpack or _G.unpack
util.seed()

local function randtable(n, a, b)
    local t = {}
    for i=1, n do
        t[i] = util.randab(a, b)
    end
    return t
end

local function randomsphere()
    local pos = vec3.new(unpack(randtable(3, -2, 2)))
    local radius = util.randab(0, 0.5)
	local emission = colorvec.new(unpack(randtable(4, 0, 1)))
    local reflection = colormat.new(unpack(randtable(16, 0, 1)))
    local surfacetype = util.choice({surfacetype.DIFFUSE, surfacetype.REFLECTIVE, surfacetype.STOP})

    return obj.withmaterial(
        obj.sphere(pos, radius),
        material.new(emission, reflection, surfacetype)
    )
end

local spheres = {}
for i=1, 100 do
    spheres[i] = randomsphere()
end
return util.union(spheres)