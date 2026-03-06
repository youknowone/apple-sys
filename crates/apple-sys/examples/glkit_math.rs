//! Quaternion and matrix math with GLKit.
//!
//! Uses GLKQuaternion functions and GLKMatrix4 inversion
//! for 3D rotation math.

use apple_sys::GLKit::*;

fn print_matrix(label: &str, m: &GLKMatrix4) {
    println!("{}:", label);
    let v = unsafe { &m.__bindgen_anon_1 };
    println!(
        "  [{:8.4} {:8.4} {:8.4} {:8.4}]",
        v.m00, v.m01, v.m02, v.m03
    );
    println!(
        "  [{:8.4} {:8.4} {:8.4} {:8.4}]",
        v.m10, v.m11, v.m12, v.m13
    );
    println!(
        "  [{:8.4} {:8.4} {:8.4} {:8.4}]",
        v.m20, v.m21, v.m22, v.m23
    );
    println!(
        "  [{:8.4} {:8.4} {:8.4} {:8.4}]",
        v.m30, v.m31, v.m32, v.m33
    );
}

fn main() {
    unsafe {
        println!("=== GLKit Math ===\n");

        // Identity matrix
        let identity = GLKMatrix4Identity;
        print_matrix("Identity", &identity);

        // Invert identity (should be identity)
        let mut invertible = false;
        let inv = GLKMatrix4Invert(identity, &mut invertible);
        println!("\nIdentity invertible: {}", invertible);
        print_matrix("Inverse of identity", &inv);

        // Create a quaternion from matrix
        let quat = GLKQuaternionMakeWithMatrix4(identity);
        let angle = GLKQuaternionAngle(quat);
        let axis = GLKQuaternionAxis(quat);
        println!(
            "\nQuaternion from identity: angle={:.4}, axis=({:.4}, {:.4}, {:.4})",
            angle, axis.__bindgen_anon_1.x, axis.__bindgen_anon_1.y, axis.__bindgen_anon_1.z
        );

        // Create a rotation matrix manually (90° around Z)
        let angle_rad = std::f32::consts::FRAC_PI_2;
        let c = angle_rad.cos();
        let s = angle_rad.sin();
        let rot_z = _GLKMatrix4 {
            __bindgen_anon_1: _GLKMatrix4__bindgen_ty_1 {
                m00: c,
                m01: s,
                m02: 0.0,
                m03: 0.0,
                m10: -s,
                m11: c,
                m12: 0.0,
                m13: 0.0,
                m20: 0.0,
                m21: 0.0,
                m22: 1.0,
                m23: 0.0,
                m30: 0.0,
                m31: 0.0,
                m32: 0.0,
                m33: 1.0,
            },
        };
        print_matrix("\nRotation Z(90°)", &rot_z);

        // Extract quaternion from rotation
        let quat = GLKQuaternionMakeWithMatrix4(rot_z);
        let angle = GLKQuaternionAngle(quat);
        let axis = GLKQuaternionAxis(quat);
        println!(
            "Quaternion: angle={:.4} rad ({:.1}°), axis=({:.4}, {:.4}, {:.4})",
            angle,
            angle.to_degrees(),
            axis.__bindgen_anon_1.x,
            axis.__bindgen_anon_1.y,
            axis.__bindgen_anon_1.z
        );

        // SLERP between two quaternions
        let q_identity = GLKQuaternionMakeWithMatrix4(identity);
        let q_half = GLKQuaternionSlerp(q_identity, quat, 0.5);
        let half_angle = GLKQuaternionAngle(q_half);
        println!(
            "SLERP(identity, rot90, 0.5): angle={:.4} rad ({:.1}°)",
            half_angle,
            half_angle.to_degrees()
        );

        // Invert the rotation matrix
        let mut invertible = false;
        let inv = GLKMatrix4Invert(rot_z, &mut invertible);
        println!("\nRotation invertible: {}", invertible);
        print_matrix("Inverse rotation Z(90°)", &inv);

        // InvertAndTranspose
        let inv_trans = GLKMatrix4InvertAndTranspose(rot_z, &mut invertible);
        print_matrix("\nInverseTranspose rotation Z(90°)", &inv_trans);

        // 3x3 matrix invert
        let m3 = _GLKMatrix3 {
            __bindgen_anon_1: _GLKMatrix3__bindgen_ty_1 {
                m00: 1.0,
                m01: 0.0,
                m02: 0.0,
                m10: 0.0,
                m11: 2.0,
                m12: 0.0,
                m20: 0.0,
                m21: 0.0,
                m22: 3.0,
            },
        };
        let inv3 = GLKMatrix3Invert(m3, &mut invertible);
        println!("\n3x3 scale(1,2,3) invertible: {}", invertible);
        let v = inv3.__bindgen_anon_1;
        println!(
            "  Inverse diagonal: ({:.4}, {:.4}, {:.4})",
            v.m00, v.m11, v.m22
        );
    }

    println!("\nDone.");
}
