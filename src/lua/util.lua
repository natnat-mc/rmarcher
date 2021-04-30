local env = ...
local obj = env.obj
local vec3 = env.vec3
local mat3 = env.mat3
local transform = env.transform

local M = {}

-- technically a binary fold
local function binstack(objs, fn)
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
	return fn(binstack(left, fn), binstack(right, fn))
end

-- technically a fold
local function linstack(objs, fn)
	local head = objs[1]
	for i=2, #objs do
		head = fn(head, objs[i])
	end
	return head
end

-- random functions
local SEED = os.getenv "SEED" or os.time()
function M.seed()
	math.randomseed(SEED)
end
function M.randab(a, b)
	return math.random() * (b-a) + a
end
function M.choice(list)
	return list[math.random(1, #list)]
end

-- multiple object composition
function M.union(objs)
	return binstack(objs, obj.union)
end
function M.intersection(objs)
	return binstack(objs, obj.intersection)
end
function M.smoothunion(objs, k)
	return linstack(objs, function(a, b) return obj.smoothunion(a, b, k) end)
end
function M.smoothintersection(objs, k)
	return linstack(objs, function(a, b) return obj.smoothintersection(a, b, k) end)
end

-- simpler transformations
function M.scale(object, k)
	return obj.affinetransform(object, transform.scale(k), vec3.O)
end
function M.transform(object, mat)
	return obj.affinetransform(object, mat, vec3.O)
end
function M.translate(object, vec)
	return obj.affinetransform(object, mat3.O, vec)
end

return M