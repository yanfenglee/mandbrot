// shader.frag
#version 450
precision mediump float;

layout(location=0) out vec4 f_color;

layout(set = 0, binding = 0)
uniform Setting {
    float centerx;
    float centery;
    float scale;
};

const vec2 resolution = vec2(1024,1024);
vec2 center = vec2(centerx-512.0, centery-512.0)/1024.0;

vec2 f(vec2 z, vec2 c) {
	return vec2(z.x*z.x-z.y*z.y, 2*z.x*z.y) + c;
}



void main() {
    vec2 uv = gl_FragCoord.xy / resolution;

    vec2 c = (center + uv) * scale;
  
    vec2 z = vec2(0.0);
    bool diverge = false;
    for (int i = 0; i < 100; i++) {
        z = f(z, c);
        if (length(z) > 2.0) {
            diverge = true;
            break;
        }
    }

    f_color = diverge ? vec4(1.0) : vec4(vec3(0.0), 1.0);
}