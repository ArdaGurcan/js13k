pub const SHADER: &str = r#"
    attribute vec4 aPosition;
    uniforn mat4 uTransform;

    void main() {
        gl_Position = uTransform * aPosition;
    }
"#;"