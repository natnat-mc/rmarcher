local unpack = table.unpack or _G.unpack
math.randomseed(os.time())

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

local function randomsphere()
    local pos = vec3.new(unpack(randtable(3, -2, 2)))
    local radius = randab(0, 0.5)
	local emission = colorvec.new(unpack(randtable(4, 0, 1)))
    local reflection = colormat.new(unpack(randtable(16, 0, 1)))
    local surfacetype = choice({surfacetype.DIFFUSE, surfacetype.REFLECTIVE, surfacetype.STOP})

    return obj.withmaterial(
        obj.sphere(pos, radius),
        material.new(emission, reflection, surfacetype)
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

local spheres = {}
for i=1, 100 do
    spheres[i] = randomsphere()
end
return union(spheres)