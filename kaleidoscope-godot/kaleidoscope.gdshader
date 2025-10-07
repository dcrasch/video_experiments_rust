shader_type canvas_item;

uniform sampler2D u_tex;
uniform vec2 in_size = vec2(1.0, 1.0);
uniform vec2 out_size = vec2(2.0, 2.0);
uniform vec2 center_in = vec2(0.5, 0.5);
uniform vec2 center_out = vec2(1.0, 1.0);
uniform float r_start = 0.0;
uniform float r_out = 0.0;
uniform float width = 0.5;
uniform float base_scale = 8.0;
uniform float u_time = 0.0;

void fragment() {
    vec2 frag_coord = UV * out_size;
    float dx = frag_coord.x - center_out.x;
    float dy = frag_coord.y - center_out.y;

    float radius = length(vec2(dx, dy));
    float theta = atan(dy, dx);

    // pulsate
    float pulse = 0.8 + 0.2 * sin(u_time * 2.0 - radius * 0.05);
    float mag_p = radius / (base_scale * pulse);

	// get color from source texture
    float theta_p = r_start + abs(mod(theta - r_start - r_out, 2.0 * width) - width);
    float src_x = mag_p * cos(theta_p) + center_in.x;
    float src_y = mag_p * sin(theta_p) + center_in.y;
    vec2 src_uv = vec2(src_x, src_y) / in_size;
    vec4 texColor;
    if (src_uv.x < 0.0 || src_uv.x > 1.0 || src_uv.y < 0.0 || src_uv.y > 1.0) {
        texColor = vec4(0.0, 0.0, 0.0, 1.0);
    } else {
        texColor = texture(u_tex, src_uv);
    }

    COLOR = texColor;
}
