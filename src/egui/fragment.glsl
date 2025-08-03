// Fragment Shader Pseudocode
void main() {
    // Get the color from the texture at the given UV coordinate
    vec4 textureColor = texture(myTexture, fragment_uv);
    // Multiply it by the vertex color
    gl_FragColor = textureColor * vertexColor;
}
