#version 330 core

out vec4 Color;
in vec3 normal;
in vec3 frag_pos;
in vec3 uv_out;
uniform sampler2D lava_texture;
uniform vec3 pos_cam;

float compute_diffuse_light(vec3 normal, vec3 light_dir){

    float intensity = max(0, dot(normal, light_dir));
    return intensity;
}

vec3 compute_specular_light(vec3 normals, vec3 light_dir, vec3 light_color){
    float specular_strength = 0.8;
    vec3 view_dir = normalize(pos_cam - frag_pos);
    vec3 reflect_dir = reflect(-light_dir, normals);
    float shininess = 32;
    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), shininess);
    vec3 specular = specular_strength * spec * light_color;
    return specular;
}
void main()
{
    vec3 light_color = vec3(1, 1, 1);
    float ambient = 0.2;
    vec3 norm = normalize(normal);
    vec3 light_pos = vec3(1, 1, 1);
    vec3 light_dir = normalize(light_pos - frag_pos);
    float diffuse = compute_diffuse_light(norm, light_dir);

    vec4 object_color = texture(lava_texture, uv_out.xy);
    vec3 specular = compute_specular_light(norm, light_dir, light_color);
    vec3 ambient_color = ambient*light_color;
    vec3 diffuse_color = diffuse * light_color;
    Color = vec4((ambient_color+diffuse_color+specular), 1)*object_color;
}