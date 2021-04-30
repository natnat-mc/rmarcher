local mirror = obj.withmaterial(
	obj.negation(
		obj.sphere(vec3.O, 8)
	),
	material.newfromdiagonal(
		colorvec.new(0, 0, 0, 0),
		colorvec.new(1, 1, 1, 1),
		surfacetype.REFLECTIVE
	)
)

local objects = {
	obj.withmaterial(
		obj.torus(vec3.new(1, 0, -3), 2, 1),
		material.newfromdiagonal(
			colorvec.new(1, .5, .2, 0),
			colorvec.new(1, 1, 1, 1),
			surfacetype.DIFFUSE
		)
	),
	obj.withmaterial(
		obj.sphere(vec3.new(3, 2, 0), 1),
		material.newfromdiagonal(
			colorvec.new(0, 0, 0, 0),
			colorvec.new(.2, .5, 1, 0),
			surfacetype.DIFFUSE
		)
	),
	obj.withmaterial(
		obj.cuboid(vec3.new(-3, -1, 2), vec3.new(1, 1, 1)),
		material.newfromdiagonal(
			colorvec.new(0, 0, 0, 0),
			colorvec.new(.2, 1, .5, 0),
			surfacetype.DIFFUSE
		)
	)
}

return obj.union(
	util.union(objects),
	mirror
)