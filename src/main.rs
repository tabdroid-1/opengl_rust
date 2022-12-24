#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
extern crate glfw;
use self::glfw::Context;

extern crate gl;
use self::gl::types::*;

use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::ffi::CStr;

pub mod tabby;
pub mod macros;

use crate::tabby::core::common::{process_events, processInput, loadTexture};
use crate::tabby::core::shader::Shader;
use crate::tabby::core::camera::Camera;
use crate::tabby::models::model::Model;

use cgmath::{Matrix4, Vector3, vec3, Point3, Deg, perspective};
use cgmath::prelude::*;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main() {
  let mut camera = Camera {
      Position: Point3::new(0.0, 0.0, 3.0),
      MovementSpeed: 100.0,
      ..Camera::default()
  };

  let mut firstMouse = true;
  let mut lastX: f32 = SCR_WIDTH as f32 / 2.0;
  let mut lastY: f32 = SCR_HEIGHT as f32 / 2.0;

  // timing
  let mut deltaTime: f32; // time between current frame and last frame
  let mut lastFrame: f32 = 0.0;

  // glfw: initialize and configure
  // ------------------------------
  let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
  glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
  glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
  #[cfg(target_os = "macos")]
  glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

  glfw::SwapInterval::Sync(0);

  // glfw window creation
  // --------------------
  let (mut window, events) = glfw.create_window(SCR_WIDTH, SCR_HEIGHT, "TabbyE", glfw::WindowMode::Windowed)
      .expect("Failed to create GLFW window");

  window.make_current();
  window.set_framebuffer_size_polling(true);
  window.set_cursor_pos_polling(true);
  window.set_scroll_polling(true);

  // tell GLFW to capture our mouse
  window.set_cursor_mode(glfw::CursorMode::Disabled);

  // gl: load all OpenGL function pointers
  // ---------------------------------------
  gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

  let (ourShader, ourModel) = unsafe {
      // configure global opengl state
      // -----------------------------
      gl::Enable(gl::DEPTH_TEST);

      // build and compile shaders
      // -------------------------
      let ourShader = Shader::new(
          "assets/shaders/model.vs",
          "assets/shaders/model.fs");

      // load models
      // -----------
      let ourModel = Model::new("assets/objects/nanosuit/nanosuit.obj");

      // draw in wireframe
      // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

      (ourShader, ourModel)
  };

  // render loop
  // -----------

  let mut models = [Matrix4::<f32>::from_translation(vec3(0.0, -1.75, 0.0)); 2000]; 
  while !window.should_close() {

      // per-frame time logic
      // --------------------
      let currentFrame = glfw.get_time() as f32;
      deltaTime = currentFrame - lastFrame;
      lastFrame = currentFrame;

      let fps = 1000 as f32 / deltaTime;

      println!("{fps}");

      // events
      // -----
      process_events(&events, &mut firstMouse, &mut lastX, &mut lastY, &mut camera);

      // input
      // -----
      processInput(&mut window, deltaTime, &mut camera);

      // render
      // ------
      unsafe {
          gl::ClearColor(0.4, 0.1, 0.8, 1.0);
          gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

          // don't forget to enable shader before setting uniforms
          ourShader.useProgram();

          // view/projection transformations
          let projection: Matrix4<f32> = perspective(Deg(camera.Zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
          let view = camera.GetViewMatrix();
          ourShader.setMat4(c_str!("projection"), &projection);
          ourShader.setMat4(c_str!("view"), &view);

          // render the loaded model
          //let mut model = Matrix4::<f32>::from_translation(vec3(0.0, -1.75, 0.0)); // translate it down so it's at the center of the scene

          let mut amount1 = 0;
          let mut amount2 = 0;

          for mut model in models {

            model = Matrix4::<f32>::from_translation(vec3((2 * amount2) as f32, -1.75, (2 * amount1) as f32));
            model = model * Matrix4::from_scale(0.2);  // it's a bit too big for our scene, so scale it down
            ourShader.setMat4(c_str!("model"), &model);
            ourModel.Draw(&ourShader);

            amount2 += 1;

            if amount2 % 40 == 0 {
              amount1 += 1;
              amount2 = 0;
            }
          }        
      }

      // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
      // -------------------------------------------------------------------------------
      window.swap_buffers();
      glfw.poll_events();
  }

}