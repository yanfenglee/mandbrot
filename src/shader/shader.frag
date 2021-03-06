// shader.frag
#version 450

layout(location=0) out vec4 f_color;

const vec2 resolution = vec2(1024,1024);
vec2 center = vec2(0,0);
vec2 scale = vec2(1,1);

vec2 f(vec2 z, vec2 c) {
	return vec2(z.x*z.x-z.y*z.y, 2*z.x*z.y) + c;
}



void main() {
    vec2 uv = gl_FragCoord.xy / resolution;

    vec2 c = center + uv * scale;
  
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