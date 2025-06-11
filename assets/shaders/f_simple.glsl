#version 330

in vec3 FragPos;
in vec3 Normal;
in vec2 TexCoord;

out vec4 pixelColor;

uniform vec3 lightPos = vec3(0.0, 0.0, 2.0); 
uniform sampler2D baseColorTexture;
uniform sampler2D normalTexture;
uniform sampler2D emissiveTexture;

void main(void) {
    // Base color
    vec3 baseColor = texture(baseColorTexture, TexCoord).rgb;

    // Normals from normal map (in tangent space)
    vec3 normalMap = texture(normalTexture, TexCoord).rgb;
    vec3 norm = normalize(normalMap * 2.0 - 1.0); // unpack from [0,1] to [-1,1]

    // Lighting
    vec3 lightDir = normalize(lightPos - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * baseColor;

    vec3 ambient = 0.2 * baseColor;

    vec3 viewDir = normalize(-FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 16.0);
    vec3 specular = 0.5 * spec * vec3(1.0);

    // Emissive color
    vec3 emissive = texture(emissiveTexture, TexCoord).rgb;

    // Final color
    vec3 result = ambient + diffuse + specular + emissive;
    pixelColor = vec4(result, 1.0);
}
