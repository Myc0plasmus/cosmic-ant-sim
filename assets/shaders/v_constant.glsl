#version 330

//Zmienne jednorodne
uniform mat4 P;
uniform mat4 V;
uniform mat4 M;

//Atrybuty
layout (location=0) in vec4 vertex; //wspolrzedne wierzcholka w przestrzeni modelu
layout (location=1) in vec4 normal; //wektor normalny w wierzcholku
// layout (location=2) in vec2 texCoord; //wspó³rzêdne teksturowania
// layout (location=3) in vec4 color; //kolor wierzcho³ka
// layout (location=4) in vec4 vertex_normal; //kolor wierzcho³ka


void main(void) {
    gl_Position=P*V*M*vertex;
}
