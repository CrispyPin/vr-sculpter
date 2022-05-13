shader_type spatial;


void vertex() {
	float norm_i = fract(VERTEX.x);
	if (norm_i < .01) {
		NORMAL = vec3(1., 0., 0.);
	}
	else if (norm_i < .02) {
		NORMAL = vec3(-1., 0., 0.);
	}
	else if (norm_i < .03) {
		NORMAL = vec3(0., 1., 0.);
	}
	else if (norm_i < .04) {
		NORMAL = vec3(0., -1., 0.);
	}
	else if (norm_i < .05) {
		NORMAL = vec3(0., 0., 1.);
	}
	else {
		NORMAL = vec3(0., 0., -1.);
	}
	UV2 = fract(VERTEX.zy) * 100.0;
	VERTEX = floor(VERTEX);
}

void fragment() {
	// vec3 pos = (CAMERA_MATRIX * vec4(VERTEX, 1.0)).xyz;
	// pos = fract(pos);
	vec3 normal = (CAMERA_MATRIX * vec4(NORMAL, 0.0)).xyz;

	float uv_vis = 1.0;
	if (UV2.x > 0.95 ||
		UV2.x < 0.05 ||
		UV2.y > 0.95 ||
		UV2.y < 0.05
		|| abs(UV2.x - UV2.y) < 0.02
		|| abs((1.0 - UV2.x) - UV2.y) < 0.02
		) {
		uv_vis = 0.5;
	}

	vec3 color = vec3(0.3, 0.3, 0.3);
	if (normal.y > 0.9) {
		color = vec3(0.1,0.9,0.3);
	}
	else {
		uv_vis *= 0.8;
	}
	ALBEDO = color * uv_vis;
	// ALBEDO = uv_vis * vec3(0.4, 0.5, 0.8);
}
