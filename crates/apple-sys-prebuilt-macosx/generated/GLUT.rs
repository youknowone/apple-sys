#[allow(unused_imports)]
use crate::OpenGL::*;

pub type gleVector = [f64; 3usize];
pub type SphereMapFlags = ::std::os::raw::c_uint;
pub type SphereMap = _SphereMap;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _STXY {
    pub s: GLfloat,
    pub t: GLfloat,
    pub x: GLfloat,
    pub y: GLfloat,
}
pub type STXY = _STXY;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _SphereMapMesh {
    pub refcnt: ::std::os::raw::c_int,
    pub steps: ::std::os::raw::c_int,
    pub rings: ::std::os::raw::c_int,
    pub edgeExtend: ::std::os::raw::c_int,
    pub face: *mut STXY,
    pub back: *mut STXY,
}
pub type SphereMapMesh = _SphereMapMesh;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _SphereMap {
    pub mesh: *mut SphereMapMesh,
    pub smapTexObj: GLuint,
    pub viewTexObjs: [GLuint; 6usize],
    pub viewTexObj: GLuint,
    pub flags: SphereMapFlags,
    pub viewTexDim: ::std::os::raw::c_int,
    pub smapTexDim: ::std::os::raw::c_int,
    pub viewOrigin: [::std::os::raw::c_int; 2usize],
    pub smapOrigin: [::std::os::raw::c_int; 2usize],
    pub eye: [GLfloat; 3usize],
    pub up: [GLfloat; 3usize],
    pub obj: [GLfloat; 3usize],
    pub viewNear: GLfloat,
    pub viewFar: GLfloat,
    pub positionLights: ::std::option::Option<
        unsafe extern "C" fn(view: ::std::os::raw::c_int, context: *mut ::std::os::raw::c_void),
    >,
    pub drawView: ::std::option::Option<
        unsafe extern "C" fn(view: ::std::os::raw::c_int, context: *mut ::std::os::raw::c_void),
    >,
    pub context: *mut ::std::os::raw::c_void,
}
pub type gleAffine = [[f64; 3usize]; 2usize];
pub type gleColor = [f32; 3usize];
pub type gleTwoVec = [f64; 2usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gleGC {
    pub bgn_gen_texture:
        ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int, arg2: f64)>,
    pub n3f_gen_texture: ::std::option::Option<unsafe extern "C" fn(arg1: *mut f32)>,
    pub n3d_gen_texture: ::std::option::Option<unsafe extern "C" fn(arg1: *mut f64)>,
    pub v3f_gen_texture: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut f32,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
        ),
    >,
    pub v3d_gen_texture: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut f64,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
        ),
    >,
    pub end_gen_texture: ::std::option::Option<unsafe extern "C" fn()>,
    pub join_style: ::std::os::raw::c_int,
    pub ncp: ::std::os::raw::c_int,
    pub contour: *mut gleTwoVec,
    pub cont_normal: *mut gleTwoVec,
    pub up: *mut f64,
    pub npoints: ::std::os::raw::c_int,
    pub point_array: *mut gleVector,
    pub color_array: *mut gleColor,
    pub xform_array: *mut gleAffine,
    pub num_vert: ::std::os::raw::c_int,
    pub segment_number: ::std::os::raw::c_int,
    pub segment_length: f64,
    pub accum_seg_len: f64,
    pub prev_x: f64,
    pub prev_y: f64,
    pub save_bgn_gen_texture:
        ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int, arg2: f64)>,
    pub save_n3f_gen_texture: ::std::option::Option<unsafe extern "C" fn(arg1: *mut f32)>,
    pub save_n3d_gen_texture: ::std::option::Option<unsafe extern "C" fn(arg1: *mut f64)>,
    pub save_v3f_gen_texture: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut f32,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
        ),
    >,
    pub save_v3d_gen_texture: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut f64,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
        ),
    >,
    pub save_end_gen_texture: ::std::option::Option<unsafe extern "C" fn()>,
}
unsafe extern "C" {
    pub static mut glutStrokeRoman: *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub static mut glutStrokeMonoRoman: *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub static mut glutBitmap9By15: *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub static mut glutBitmap8By13: *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub static mut glutBitmapTimesRoman10: *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub static mut glutBitmapTimesRoman24: *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub static mut glutBitmapHelvetica10: *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub static mut glutBitmapHelvetica12: *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub static mut glutBitmapHelvetica18: *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn glutInit(argcp: *mut ::std::os::raw::c_int, argv: *mut *mut ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn glutInitDisplayMode(mode: ::std::os::raw::c_uint);
}
unsafe extern "C" {
    pub fn glutInitDisplayString(string: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn glutInitWindowPosition(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn glutInitWindowSize(width: ::std::os::raw::c_int, height: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn glutMainLoop();
}
unsafe extern "C" {
    pub fn glutCreateWindow(title: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn glutCreateSubWindow(
        win: ::std::os::raw::c_int,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn glutDestroyWindow(win: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn glutPostRedisplay();
}
unsafe extern "C" {
    pub fn glutPostWindowRedisplay(win: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn glutSwapBuffers();
}
unsafe extern "C" {
    pub fn glutGetWindow() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn glutSetWindow(win: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn glutSetWindowTitle(title: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn glutSetIconTitle(title: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn glutPositionWindow(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn glutReshapeWindow(width: ::std::os::raw::c_int, height: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn glutPopWindow();
}
unsafe extern "C" {
    pub fn glutPushWindow();
}
unsafe extern "C" {
    pub fn glutIconifyWindow();
}
unsafe extern "C" {
    pub fn glutShowWindow();
}
unsafe extern "C" {
    pub fn glutHideWindow();
}
unsafe extern "C" {
    pub fn glutFullScreen();
}
unsafe extern "C" {
    pub fn glutSetCursor(cursor: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn glutWarpPointer(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn glutSurfaceTexture(
        target: GLenum,
        internalformat: GLenum,
        surfacewin: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn glutWMCloseFunc(func: ::std::option::Option<unsafe extern "C" fn()>);
}
unsafe extern "C" {
    pub fn glutCheckLoop();
}
unsafe extern "C" {
    pub fn glutEstablishOverlay();
}
unsafe extern "C" {
    pub fn glutRemoveOverlay();
}
unsafe extern "C" {
    pub fn glutUseLayer(layer: GLenum);
}
unsafe extern "C" {
    pub fn glutPostOverlayRedisplay();
}
unsafe extern "C" {
    pub fn glutPostWindowOverlayRedisplay(win: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn glutShowOverlay();
}
unsafe extern "C" {
    pub fn glutHideOverlay();
}
unsafe extern "C" {
    pub fn glutCreateMenu(
        arg1: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn glutDestroyMenu(menu: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn glutGetMenu() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn glutSetMenu(menu: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn glutAddMenuEntry(label: *const ::std::os::raw::c_char, value: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn glutAddSubMenu(label: *const ::std::os::raw::c_char, submenu: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn glutChangeToMenuEntry(
        item: ::std::os::raw::c_int,
        label: *const ::std::os::raw::c_char,
        value: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn glutChangeToSubMenu(
        item: ::std::os::raw::c_int,
        label: *const ::std::os::raw::c_char,
        submenu: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn glutRemoveMenuItem(item: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn glutAttachMenu(button: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn glutDetachMenu(button: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn glutDisplayFunc(func: ::std::option::Option<unsafe extern "C" fn()>);
}
unsafe extern "C" {
    pub fn glutReshapeFunc(
        func: ::std::option::Option<
            unsafe extern "C" fn(width: ::std::os::raw::c_int, height: ::std::os::raw::c_int),
        >,
    );
}
unsafe extern "C" {
    pub fn glutKeyboardFunc(
        func: ::std::option::Option<
            unsafe extern "C" fn(
                key: ::std::os::raw::c_uchar,
                x: ::std::os::raw::c_int,
                y: ::std::os::raw::c_int,
            ),
        >,
    );
}
unsafe extern "C" {
    pub fn glutMouseFunc(
        func: ::std::option::Option<
            unsafe extern "C" fn(
                button: ::std::os::raw::c_int,
                state: ::std::os::raw::c_int,
                x: ::std::os::raw::c_int,
                y: ::std::os::raw::c_int,
            ),
        >,
    );
}
unsafe extern "C" {
    pub fn glutMotionFunc(
        func: ::std::option::Option<
            unsafe extern "C" fn(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int),
        >,
    );
}
unsafe extern "C" {
    pub fn glutPassiveMotionFunc(
        func: ::std::option::Option<
            unsafe extern "C" fn(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int),
        >,
    );
}
unsafe extern "C" {
    pub fn glutEntryFunc(
        func: ::std::option::Option<unsafe extern "C" fn(state: ::std::os::raw::c_int)>,
    );
}
unsafe extern "C" {
    pub fn glutVisibilityFunc(
        func: ::std::option::Option<unsafe extern "C" fn(state: ::std::os::raw::c_int)>,
    );
}
unsafe extern "C" {
    pub fn glutIdleFunc(func: ::std::option::Option<unsafe extern "C" fn()>);
}
unsafe extern "C" {
    pub fn glutTimerFunc(
        millis: ::std::os::raw::c_uint,
        func: ::std::option::Option<unsafe extern "C" fn(value: ::std::os::raw::c_int)>,
        value: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn glutMenuStateFunc(
        func: ::std::option::Option<unsafe extern "C" fn(state: ::std::os::raw::c_int)>,
    );
}
unsafe extern "C" {
    pub fn glutSpecialFunc(
        func: ::std::option::Option<
            unsafe extern "C" fn(
                key: ::std::os::raw::c_int,
                x: ::std::os::raw::c_int,
                y: ::std::os::raw::c_int,
            ),
        >,
    );
}
unsafe extern "C" {
    pub fn glutSpaceballMotionFunc(
        func: ::std::option::Option<
            unsafe extern "C" fn(
                x: ::std::os::raw::c_int,
                y: ::std::os::raw::c_int,
                z: ::std::os::raw::c_int,
            ),
        >,
    );
}
unsafe extern "C" {
    pub fn glutSpaceballRotateFunc(
        func: ::std::option::Option<
            unsafe extern "C" fn(
                x: ::std::os::raw::c_int,
                y: ::std::os::raw::c_int,
                z: ::std::os::raw::c_int,
            ),
        >,
    );
}
unsafe extern "C" {
    pub fn glutSpaceballButtonFunc(
        func: ::std::option::Option<
            unsafe extern "C" fn(button: ::std::os::raw::c_int, state: ::std::os::raw::c_int),
        >,
    );
}
unsafe extern "C" {
    pub fn glutButtonBoxFunc(
        func: ::std::option::Option<
            unsafe extern "C" fn(button: ::std::os::raw::c_int, state: ::std::os::raw::c_int),
        >,
    );
}
unsafe extern "C" {
    pub fn glutDialsFunc(
        func: ::std::option::Option<
            unsafe extern "C" fn(dial: ::std::os::raw::c_int, value: ::std::os::raw::c_int),
        >,
    );
}
unsafe extern "C" {
    pub fn glutTabletMotionFunc(
        func: ::std::option::Option<
            unsafe extern "C" fn(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int),
        >,
    );
}
unsafe extern "C" {
    pub fn glutTabletButtonFunc(
        func: ::std::option::Option<
            unsafe extern "C" fn(
                button: ::std::os::raw::c_int,
                state: ::std::os::raw::c_int,
                x: ::std::os::raw::c_int,
                y: ::std::os::raw::c_int,
            ),
        >,
    );
}
unsafe extern "C" {
    pub fn glutMenuStatusFunc(
        func: ::std::option::Option<
            unsafe extern "C" fn(
                status: ::std::os::raw::c_int,
                x: ::std::os::raw::c_int,
                y: ::std::os::raw::c_int,
            ),
        >,
    );
}
unsafe extern "C" {
    pub fn glutOverlayDisplayFunc(func: ::std::option::Option<unsafe extern "C" fn()>);
}
unsafe extern "C" {
    pub fn glutWindowStatusFunc(
        func: ::std::option::Option<unsafe extern "C" fn(state: ::std::os::raw::c_int)>,
    );
}
unsafe extern "C" {
    pub fn glutKeyboardUpFunc(
        func: ::std::option::Option<
            unsafe extern "C" fn(
                key: ::std::os::raw::c_uchar,
                x: ::std::os::raw::c_int,
                y: ::std::os::raw::c_int,
            ),
        >,
    );
}
unsafe extern "C" {
    pub fn glutSpecialUpFunc(
        func: ::std::option::Option<
            unsafe extern "C" fn(
                key: ::std::os::raw::c_int,
                x: ::std::os::raw::c_int,
                y: ::std::os::raw::c_int,
            ),
        >,
    );
}
unsafe extern "C" {
    pub fn glutJoystickFunc(
        func: ::std::option::Option<
            unsafe extern "C" fn(
                buttonMask: ::std::os::raw::c_uint,
                x: ::std::os::raw::c_int,
                y: ::std::os::raw::c_int,
                z: ::std::os::raw::c_int,
            ),
        >,
        pollInterval: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn glutSetColor(arg1: ::std::os::raw::c_int, red: GLfloat, green: GLfloat, blue: GLfloat);
}
unsafe extern "C" {
    pub fn glutGetColor(ndx: ::std::os::raw::c_int, component: ::std::os::raw::c_int) -> GLfloat;
}
unsafe extern "C" {
    pub fn glutCopyColormap(win: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn glutGet(type_: GLenum) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn glutDeviceGet(type_: GLenum) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn glutExtensionSupported(name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn glutGetModifiers() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn glutLayerGet(type_: GLenum) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn glutGetProcAddress(
        procName: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn glutBitmapCharacter(font: *mut ::std::os::raw::c_void, character: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn glutBitmapWidth(
        font: *mut ::std::os::raw::c_void,
        character: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn glutStrokeCharacter(font: *mut ::std::os::raw::c_void, character: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn glutStrokeWidth(
        font: *mut ::std::os::raw::c_void,
        character: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn glutBitmapLength(
        font: *mut ::std::os::raw::c_void,
        string: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn glutStrokeLength(
        font: *mut ::std::os::raw::c_void,
        string: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn glutWireSphere(radius: GLdouble, slices: GLint, stacks: GLint);
}
unsafe extern "C" {
    pub fn glutSolidSphere(radius: GLdouble, slices: GLint, stacks: GLint);
}
unsafe extern "C" {
    pub fn glutWireCone(base: GLdouble, height: GLdouble, slices: GLint, stacks: GLint);
}
unsafe extern "C" {
    pub fn glutSolidCone(base: GLdouble, height: GLdouble, slices: GLint, stacks: GLint);
}
unsafe extern "C" {
    pub fn glutWireCube(size: GLdouble);
}
unsafe extern "C" {
    pub fn glutSolidCube(size: GLdouble);
}
unsafe extern "C" {
    pub fn glutWireTorus(innerRadius: GLdouble, outerRadius: GLdouble, sides: GLint, rings: GLint);
}
unsafe extern "C" {
    pub fn glutSolidTorus(innerRadius: GLdouble, outerRadius: GLdouble, sides: GLint, rings: GLint);
}
unsafe extern "C" {
    pub fn glutWireDodecahedron();
}
unsafe extern "C" {
    pub fn glutSolidDodecahedron();
}
unsafe extern "C" {
    pub fn glutWireTeapot(size: GLdouble);
}
unsafe extern "C" {
    pub fn glutSolidTeapot(size: GLdouble);
}
unsafe extern "C" {
    pub fn glutWireOctahedron();
}
unsafe extern "C" {
    pub fn glutSolidOctahedron();
}
unsafe extern "C" {
    pub fn glutWireTetrahedron();
}
unsafe extern "C" {
    pub fn glutSolidTetrahedron();
}
unsafe extern "C" {
    pub fn glutWireIcosahedron();
}
unsafe extern "C" {
    pub fn glutSolidIcosahedron();
}
unsafe extern "C" {
    pub fn glutVideoResizeGet(param: GLenum) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn glutSetupVideoResizing();
}
unsafe extern "C" {
    pub fn glutStopVideoResizing();
}
unsafe extern "C" {
    pub fn glutVideoResize(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn glutVideoPan(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn glutReportErrors();
}
unsafe extern "C" {
    pub fn glutIgnoreKeyRepeat(ignore: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn glutSetKeyRepeat(repeatMode: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn glutForceJoystickFunc();
}
unsafe extern "C" {
    pub fn glutGameModeString(string: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn glutEnterGameMode() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn glutLeaveGameMode();
}
unsafe extern "C" {
    pub fn glutGameModeGet(mode: GLenum) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn smapCreateSphereMap(shareSmap: *mut SphereMap) -> *mut SphereMap;
}
unsafe extern "C" {
    pub fn smapDestroySphereMap(smap: *mut SphereMap);
}
unsafe extern "C" {
    pub fn smapConfigureSphereMapMesh(
        smap: *mut SphereMap,
        steps: ::std::os::raw::c_int,
        rings: ::std::os::raw::c_int,
        edgeExtend: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn smapSetSphereMapTexObj(smap: *mut SphereMap, texobj: GLuint);
}
unsafe extern "C" {
    pub fn smapSetViewTexObj(smap: *mut SphereMap, texobj: GLuint);
}
unsafe extern "C" {
    pub fn smapSetViewTexObjs(smap: *mut SphereMap, texobjs: *mut GLuint);
}
unsafe extern "C" {
    pub fn smapGetSphereMapTexObj(smap: *mut SphereMap, texobj: *mut GLuint);
}
unsafe extern "C" {
    pub fn smapGetViewTexObj(smap: *mut SphereMap, texobj: *mut GLuint);
}
unsafe extern "C" {
    pub fn smapGetViewTexObjs(smap: *mut SphereMap, texobjs: *mut GLuint);
}
unsafe extern "C" {
    pub fn smapSetFlags(smap: *mut SphereMap, flags: SphereMapFlags);
}
unsafe extern "C" {
    pub fn smapGetFlags(smap: *mut SphereMap, flags: *mut SphereMapFlags);
}
unsafe extern "C" {
    pub fn smapSetViewOrigin(smap: *mut SphereMap, x: GLint, y: GLint);
}
unsafe extern "C" {
    pub fn smapSetSphereMapOrigin(smap: *mut SphereMap, x: GLint, y: GLint);
}
unsafe extern "C" {
    pub fn smapGetViewOrigin(smap: *mut SphereMap, x: *mut GLint, y: *mut GLint);
}
unsafe extern "C" {
    pub fn smapGetSphereMapOrigin(smap: *mut SphereMap, x: *mut GLint, y: *mut GLint);
}
unsafe extern "C" {
    pub fn smapSetEye(smap: *mut SphereMap, eyex: GLfloat, eyey: GLfloat, eyez: GLfloat);
}
unsafe extern "C" {
    pub fn smapSetEyeVector(smap: *mut SphereMap, eye: *mut GLfloat);
}
unsafe extern "C" {
    pub fn smapSetUp(smap: *mut SphereMap, upx: GLfloat, upy: GLfloat, upz: GLfloat);
}
unsafe extern "C" {
    pub fn smapSetUpVector(smap: *mut SphereMap, up: *mut GLfloat);
}
unsafe extern "C" {
    pub fn smapSetObject(smap: *mut SphereMap, objx: GLfloat, objy: GLfloat, objz: GLfloat);
}
unsafe extern "C" {
    pub fn smapSetObjectVector(smap: *mut SphereMap, obj: *mut GLfloat);
}
unsafe extern "C" {
    pub fn smapGetEye(
        smap: *mut SphereMap,
        eyex: *mut GLfloat,
        eyey: *mut GLfloat,
        eyez: *mut GLfloat,
    );
}
unsafe extern "C" {
    pub fn smapGetEyeVector(smap: *mut SphereMap, eye: *mut GLfloat);
}
unsafe extern "C" {
    pub fn smapGetUp(smap: *mut SphereMap, upx: *mut GLfloat, upy: *mut GLfloat, upz: *mut GLfloat);
}
unsafe extern "C" {
    pub fn smapGetUpVector(smap: *mut SphereMap, up: *mut GLfloat);
}
unsafe extern "C" {
    pub fn smapGetObject(
        smap: *mut SphereMap,
        objx: *mut GLfloat,
        objy: *mut GLfloat,
        objz: *mut GLfloat,
    );
}
unsafe extern "C" {
    pub fn smapGetObjectVector(smap: *mut SphereMap, obj: *mut GLfloat);
}
unsafe extern "C" {
    pub fn smapSetNearFar(smap: *mut SphereMap, viewNear: GLfloat, viewFar: GLfloat);
}
unsafe extern "C" {
    pub fn smapGetNearFar(smap: *mut SphereMap, viewNear: *mut GLfloat, viewFar: *mut GLfloat);
}
unsafe extern "C" {
    pub fn smapSetSphereMapTexDim(smap: *mut SphereMap, texdim: GLsizei);
}
unsafe extern "C" {
    pub fn smapSetViewTexDim(smap: *mut SphereMap, texdim: GLsizei);
}
unsafe extern "C" {
    pub fn smapGetSphereMapTexDim(smap: *mut SphereMap, texdim: *mut GLsizei);
}
unsafe extern "C" {
    pub fn smapGetViewTexDim(smap: *mut SphereMap, texdim: *mut GLsizei);
}
unsafe extern "C" {
    pub fn smapSetContextData(smap: *mut SphereMap, context: *mut ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn smapGetContextData(smap: *mut SphereMap, context: *mut *mut ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn smapSetPositionLightsFunc(
        smap: *mut SphereMap,
        positionLights: ::std::option::Option<
            unsafe extern "C" fn(view: ::std::os::raw::c_int, context: *mut ::std::os::raw::c_void),
        >,
    );
}
unsafe extern "C" {
    pub fn smapSetDrawViewFunc(
        smap: *mut SphereMap,
        drawView: ::std::option::Option<
            unsafe extern "C" fn(view: ::std::os::raw::c_int, context: *mut ::std::os::raw::c_void),
        >,
    );
}
unsafe extern "C" {
    pub fn smapGetPositionLightsFunc(
        smap: *mut SphereMap,
        positionLights: *mut ::std::option::Option<
            unsafe extern "C" fn(view: ::std::os::raw::c_int, context: *mut ::std::os::raw::c_void),
        >,
    );
}
unsafe extern "C" {
    pub fn smapGetDrawViewFunc(
        smap: *mut SphereMap,
        drawView: *mut ::std::option::Option<
            unsafe extern "C" fn(view: ::std::os::raw::c_int, context: *mut ::std::os::raw::c_void),
        >,
    );
}
unsafe extern "C" {
    pub fn smapGenViewTex(smap: *mut SphereMap, view: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn smapGenViewTexs(smap: *mut SphereMap);
}
unsafe extern "C" {
    pub fn smapGenSphereMapFromViewTexs(smap: *mut SphereMap);
}
unsafe extern "C" {
    pub fn smapGenSphereMap(smap: *mut SphereMap);
}
unsafe extern "C" {
    pub fn smapGenSphereMapWithOneViewTex(smap: *mut SphereMap);
}
unsafe extern "C" {
    pub fn smapRvecToSt(rvec: *mut f32, st: *mut f32) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn smapStToRvec(st: *mut f32, rvec: *mut f32);
}
unsafe extern "C" {
    pub fn __glutGetFCB(which: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn __glutSetFCB(which: ::std::os::raw::c_int, func: *mut ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn gleGetJoinStyle() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn gleSetJoinStyle(style: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn gleGetNumSlices() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn gleSetNumSlices(slices: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn glePolyCylinder(
        npoints: ::std::os::raw::c_int,
        point_array: *mut [f64; 3usize],
        color_array: *mut [f32; 3usize],
        radius: f64,
    );
}
unsafe extern "C" {
    pub fn glePolyCone(
        npoints: ::std::os::raw::c_int,
        point_array: *mut [f64; 3usize],
        color_array: *mut [f32; 3usize],
        radius_array: *mut f64,
    );
}
unsafe extern "C" {
    pub fn gleExtrusion(
        ncp: ::std::os::raw::c_int,
        contour: *mut [f64; 2usize],
        cont_normal: *mut [f64; 2usize],
        up: *mut f64,
        npoints: ::std::os::raw::c_int,
        point_array: *mut [f64; 3usize],
        color_array: *mut [f32; 3usize],
    );
}
unsafe extern "C" {
    pub fn gleTwistExtrusion(
        ncp: ::std::os::raw::c_int,
        contour: *mut [f64; 2usize],
        cont_normal: *mut [f64; 2usize],
        up: *mut f64,
        npoints: ::std::os::raw::c_int,
        point_array: *mut [f64; 3usize],
        color_array: *mut [f32; 3usize],
        twist_array: *mut f64,
    );
}
unsafe extern "C" {
    pub fn gleSuperExtrusion(
        ncp: ::std::os::raw::c_int,
        contour: *mut [f64; 2usize],
        cont_normal: *mut [f64; 2usize],
        up: *mut f64,
        npoints: ::std::os::raw::c_int,
        point_array: *mut [f64; 3usize],
        color_array: *mut [f32; 3usize],
        xform_array: *mut [[f64; 3usize]; 2usize],
    );
}
unsafe extern "C" {
    pub fn gleSpiral(
        ncp: ::std::os::raw::c_int,
        contour: *mut [f64; 2usize],
        cont_normal: *mut [f64; 2usize],
        up: *mut f64,
        startRadius: f64,
        drdTheta: f64,
        startZ: f64,
        dzdTheta: f64,
        startXform: *mut [f64; 3usize],
        dXformdTheta: *mut [f64; 3usize],
        startTheta: f64,
        sweepTheta: f64,
    );
}
unsafe extern "C" {
    pub fn gleLathe(
        ncp: ::std::os::raw::c_int,
        contour: *mut [f64; 2usize],
        cont_normal: *mut [f64; 2usize],
        up: *mut f64,
        startRadius: f64,
        drdTheta: f64,
        startZ: f64,
        dzdTheta: f64,
        startXform: *mut [f64; 3usize],
        dXformdTheta: *mut [f64; 3usize],
        startTheta: f64,
        sweepTheta: f64,
    );
}
unsafe extern "C" {
    pub fn gleHelicoid(
        rToroid: f64,
        startRadius: f64,
        drdTheta: f64,
        startZ: f64,
        dzdTheta: f64,
        startXform: *mut [f64; 3usize],
        dXformdTheta: *mut [f64; 3usize],
        startTheta: f64,
        sweepTheta: f64,
    );
}
unsafe extern "C" {
    pub fn gleToroid(
        rToroid: f64,
        startRadius: f64,
        drdTheta: f64,
        startZ: f64,
        dzdTheta: f64,
        startXform: *mut [f64; 3usize],
        dXformdTheta: *mut [f64; 3usize],
        startTheta: f64,
        sweepTheta: f64,
    );
}
unsafe extern "C" {
    pub fn gleScrew(
        ncp: ::std::os::raw::c_int,
        contour: *mut [f64; 2usize],
        cont_normal: *mut [f64; 2usize],
        up: *mut f64,
        startz: f64,
        endz: f64,
        twist: f64,
    );
}
unsafe extern "C" {
    pub fn gleTextureMode(mode: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub static mut _gle_gc: *mut gleGC;
}
unsafe extern "C" {
    pub fn gleCreateGC() -> *mut gleGC;
}

unsafe impl objc2::encode::RefEncode for _STXY {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _STXY {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_STXY", &[]);
}
unsafe impl objc2::encode::RefEncode for _SphereMapMesh {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _SphereMapMesh {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_SphereMapMesh", &[]);
}
unsafe impl objc2::encode::RefEncode for _SphereMap {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _SphereMap {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_SphereMap", &[]);
}
unsafe impl objc2::encode::RefEncode for gleGC {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for gleGC {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("gleGC", &[]);
}
