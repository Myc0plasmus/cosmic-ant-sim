use nalgebra_glm as glm;

pub fn vec3_to_vec4(
    v3: glm::Vec3,
) -> glm::Vec4 {
    glm::vec4(v3.x, v3.y, v3.z, 0.0)
}

pub fn vec4_to_vec3(
    v4: glm::Vec4,
) -> glm::Vec3 {
    glm::vec3(v4.x, v4.y, v4.z)
}

pub fn vec3_ref_to_vec4(
    v3: &glm::Vec3,
) -> glm::Vec4 {
    glm::vec4(v3.x, v3.y, v3.z, 0.0)
}

pub fn vec4_ref_to_vec3(
    v4: &glm::Vec4,
) -> glm::Vec3 {
    glm::vec3(v4.x, v4.y, v4.z)
}
