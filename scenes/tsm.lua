-- Scene developed by TSM71
-- Also this took 30 hours to render

local COLOR_BLACK = colorvec.new(0, 0, 0, 0)

local GRAY = material.newfromdiagonal(
	COLOR_BLACK,
	colorvec.new(0.5, 0.5, 0.5, 0.5),
	surfacetype.DIFFUSE
)
local CYAN = material.newfromdiagonal(
	COLOR_BLACK,
	colorvec.new(0, 1, 1, 0),
	surfacetype.DIFFUSE
)
local PINK = material.newfromdiagonal(
	COLOR_BLACK,
	colorvec.new(1, .5, .5, 0),
	surfacetype.DIFFUSE
)
local RED = material.newfromdiagonal(
	COLOR_BLACK,
	colorvec.new(1, 0, 0, 0),
	surfacetype.DIFFUSE
)
local MAGENTA = material.newfromdiagonal(
	COLOR_BLACK,
	colorvec.new(1, 0, 1, 0),
	surfacetype.DIFFUSE
)
local MIRROR = material.newfromdiagonal(
	COLOR_BLACK,
	colorvec.new(1, 1, 1, 1),
	surfacetype.REFLECTIVE
)
local FLUORESCENT = material.new(
	COLOR_BLACK,
	colormat.new(
		.25, 0, 0, .25,
		0, .25, 0, .75,
		0, 0, .25, 0,
		0, 0, 0, 0
	),
	surfacetype.DIFFUSE
)

--[[
obj.transformaround = function(a, b, c)
	return util.translate(
		util.transform(
			util.translate(
				a,
				-c
			),
			b
		),
		c
	)
end
--]]
--obj.transformaround = function(a, _b, _c) return a end
--obj.affinetransform = function(a, _b, _c) return a end
--obj.smoothunion = function(a, b, _r) return obj.union(a, b) end
---[[
util.stacktransforms = function(list)
	--do return list[1] end
	--do return list[#list] end
	local n = #list
	local a = mat3.I
	--for i=n, 1, -1 do
	for i=1, n do
		a = list[i] * a
		--a = a * list[i]
	end
	return a
end
--]]

return obj.withlights(
	obj.transformaround(
		obj.withlights(
			util.union({
				util.intersection({
					util.union({
						obj.withmaterial(
							obj.negation(
								util.union({
									obj.cuboid(vec3.new(0, 0, 0), vec3.new(.5, 1, 4)),
									obj.cuboid(vec3.new(0, 0, 0), vec3.new(4, 1, .5))
								})
							),
							GRAY
						),
						obj.withmaterial(
							util.intersection({
								util.union({
									obj.cuboid(vec3.new(0, 1, 0), vec3.new(.4, .1, 3.9)),
									obj.cuboid(vec3.new(0, 1, 0), vec3.new(3.9, .1, .4))
								}),
								obj.negation(
									util.union({
										obj.cuboid(vec3.new(0, 1, 0), vec3.new(.3, 1, 3.8)),
										obj.cuboid(vec3.new(0, 1, 0), vec3.new(3.8, 1, .3))
									})
								)
							}),
							FLUORESCENT
						),
						obj.smoothunion(
							obj.withmaterial(
								obj.sphere(vec3.new(0, .75, 0), .25),
								CYAN
							),
							obj.withmaterial( -- withdynamicmaterial
								obj.sphere(vec3.new(-.5, .5, 0), .25),
								MAGENTA --dynamicmaterial.missingtexture(vec3.new(-.5, .5, 0))
							),
							.5
						)
					}),
					obj.withmaterial(
						obj.negation(
							obj.cuboid(vec3.new(0, 0, 0), vec3.new(2, .25, 2))
						),
						PINK
					)
				}),
				obj.withmaterial(
					obj.cylinder(vec3.new(1, 0, 0), .375),
					RED
				),
				util.transform(
					obj.withmaterial(
						obj.torus(vec3.new(0, 0, -1), .375, .125),
						MIRROR
					),
					transform.SWAPYZ
				)
			}),
			{
				light.new(vec3.new(.45, 1-.2, -.45), colorvec.new(0, 0, 0, .1)),
				--light.new(vec3.new(.45, 1-.2, -.45), colorvec.new(0, 0, 0, 1)),
			}
		),
		util.stacktransforms({
			transform.scalexyz(1, -1, 1),
			transform.rotatez(10*math.pi/180),
		}),
		vec3.new(0, 0, -1.5)
	),
	{
		light.new(vec3.new(0, .9375, 0),  colorvec.new(.5, .25, 0, 0)),
		light.new(vec3.new(0, .9375, -.625),  colorvec.new(0, 0, 1, 0)),
		--light.new(vec3.new(0, .9375, 0),  colorvec.new(5, 2.5, 0, 0)),
		--light.new(vec3.new(0, .9375, -.625),  colorvec.new(0, 0, 10, 0)),
	}
)