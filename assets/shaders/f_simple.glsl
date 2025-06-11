#version 330

in vec3 FragPos;
in vec3 Normal;
in vec2 TexCoord;

out vec4 pixelColor;

uniform vec3 lightPos1 = vec3(0.0, 0.0, 2.0);   // positional light
uniform vec3 lightDir2 = vec3(0.5, 0.0, -1.0);  // directional light

uniform sampler2D baseColorTexture;
uniform sampler2D normalTexture;
uniform sampler2D emissiveTexture;

void main(void) {
    vec3 baseColor = texture(baseColorTexture, TexCoord).rgb;

    // Normal mapping
    vec3 normalMap = texture(normalTexture, TexCoord).rgb;
    vec3 norm = normalize(normalMap * 2.0 - 1.0); // unpack normal

    // ---- Light 1: Point light ----
    vec3 lightDir1 = normalize(lightPos1 - FragPos);
    float diff1 = max(dot(norm, lightDir1), 0.0);
    vec3 diffuse1 = diff1 * baseColor;

    vec3 reflectDir1 = reflect(-lightDir1, norm);
    vec3 viewDir = normalize(-FragPos); // assumes camera at origin
    float spec1 = pow(max(dot(viewDir, reflectDir1), 0.0), 16.0);
    vec3 specular1 = 0.5 * spec1 * vec3(1.0);

    // ---- Light 2: Directional (sun) ----
    vec3 lightDir2Norm = normalize(-lightDir2); // sun shines *from* this direction
    float diff2 = max(dot(norm, lightDir2Norm), 0.0);
    vec3 diffuse2 = diff2 * baseColor;

    vec3 reflectDir2 = reflect(-lightDir2Norm, norm);
    float spec2 = pow(max(dot(viewDir, reflectDir2), 0.0), 16.0);
    vec3 specular2 = 0.5 * spec2 * vec3(1.0);

    // Ambient and emissive
    vec3 ambient = 0.2 * baseColor;
    vec3 emissive = texture(emissiveTexture, TexCoord).rgb;

    // Final lighting
    vec3 result = ambient + diffuse1 + diffuse2 + specular1 + specular2 + emissive;
    pixelColor = vec4(result, 1.0);
}
