precision mediump float;

uniform sampler2D u_tex;
uniform vec2 in_size;
uniform vec2 out_size;
uniform vec2 center_in;
uniform vec2 center_out;
uniform float r_start;
uniform float r_out;
uniform float width;
uniform float base_scale;
uniform float u_time;

varying vec2 vTexCoord;

void main() {
    vec2 frag_coord = vTexCoord * out_size;
    float dx = frag_coord.x - center_out.x;
    float dy = frag_coord.y - center_out.y;

    float radius = length(vec2(dx, dy));
    float theta = atan(dy, dx);

    // pulsate based on distance from center
    float pulse = 0.8 + 0.2 * sin(u_time * 2.0 - radius * 0.05);
    float mag_p = radius ;// / (base_scale * pulse);

    float theta_p = r_start + abs(mod(theta - r_start - r_out, 2.0 * width) - width);

    float src_x = mag_p * cos(theta_p) + center_in.x;
    float src_y = mag_p * sin(theta_p) + center_in.y;
    vec2 src_uv = vec2(src_x, src_y) / in_size;

    vec4 texColor;
    if (src_uv.x < 0.0 || src_uv.x > 1.0 || src_uv.y < 0.0 || src_uv.y > 1.0) {
        texColor = vec4(0.0, 0.0, 0.0, 1.0);
    } else {
        texColor = texture2D(u_tex, src_uv);
    }

    // radial intensity fade
    float r_norm = radius / (base_scale * pulse);

    float intensity = 1.0 - r_norm;
    intensity = clamp(intensity, 0.0, 1.0);

    gl_FragColor = texColor; //vec4(texColor.rgb * intensity, texColor.a);
}
