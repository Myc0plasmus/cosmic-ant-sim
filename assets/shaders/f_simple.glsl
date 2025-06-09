#version 330

in vec3 FragPos;
in vec3 Normal;

out vec4 pixelColor;

uniform vec3 lightPos = vec3(0.0, 0.0, 5.0); // Light in view space
uniform vec4 objectColor = vec4(1.0, 0.5, 0.3, 1.0);

void main(void) {
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(lightPos - FragPos);

    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * objectColor.rgb;

    vec3 ambient = 0.2 * objectColor.rgb;

    vec3 viewDir = normalize(-FragPos); // eye at origin
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 16.0);
    vec3 specular = 0.5 * spec * vec3(1.0); // white highlight


    vec3 result = ambient + diffuse + specular;

    pixelColor = vec4(result, objectColor.a);
}
