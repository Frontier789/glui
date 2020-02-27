#version 420 core

uniform sampler2D ambientOcclusion;
uniform sampler2D basecolor;
uniform sampler2D metallic;
uniform sampler2D normal;
uniform sampler2D roughness;
uniform vec3 cam_dir;

in vec2 texp;
in vec3 norm;
in vec3 tangent;
in vec3 bitangent;

out vec4 color;

vec3 light_dir = normalize(vec3(-1,-2,-6));

void main()
{
    vec3 n = normalize(norm);
    vec3 t = normalize(tangent);
    vec3 b = normalize(bitangent);
    
    vec3 nm = normalize(texture(normal, texp).xyz - .5);
    
    vec3 N = normalize(mat3(t,-b,n) * nm);
    
    float dcf = dot(N, -light_dir);
    float dcn = dot(n, -light_dir);
    
    vec4 c = texture(basecolor, texp);
    
    vec3 r = reflect(light_dir, N);
    float dr = max(dot(r, -cam_dir),0);
    
    vec3 met = texture(metallic, texp).xyz;
    
    float rough = texture(roughness, texp).x;
    rough = rough * 20;
    
    vec3 ambient  = vec3(.05);
    vec3 diffuse  = max(c.xyz * dcf * .5,0);
    vec3 specular = met * vec3(pow(dr,rough));
    
    color = vec4(ambient + diffuse * texture(ambientOcclusion, texp).x + specular, 1);
}