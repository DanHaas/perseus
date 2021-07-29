// This file contains logic to define how templates are rendered

use crate::errors::*;
use serde::{Serialize, de::DeserializeOwned};
use sycamore::prelude::{Template as SycamoreTemplate, GenericNode};

// A series of closure types that should not be typed out more than once
pub type TemplateFn<Props, G> = Box<dyn Fn(Option<Props>) -> SycamoreTemplate<G>>;
pub type GetBuildPathsFn = Box<dyn Fn() -> Vec<String>>;
pub type GetBuildStateFn<Props> = Box<dyn Fn(String) -> Props>;
pub type GetRequestStateFn<Props> = Box<dyn Fn(String) -> Props>;
pub type ShouldRevalidateFn = Box<dyn Fn() -> bool>;

/// This allows the specification of all the template templates in an app and how to render them. If no rendering logic is provided at all,
/// the template will be prerendered at build-time with no state. All closures are stored on the heap to avoid hellish lifetime specification.
// #[derive(Clone)]
pub struct Template<Props: Serialize + DeserializeOwned, G: GenericNode>
{
    /// The path to the root of the template. Any build paths will be inserted under this.
    path: String,
    /// A function that will render your template. This will be provided the rendered properties, and will be used whenever your template needs
    /// to be prerendered in some way. This should be very similar to the function that hydrates your template on the client side.
    /// This will be executed inside `sycamore::render_to_string`, and should return a `Template<SsrNode>`. This takes an `Option<Props>`
    /// because otherwise efficient typing is almost impossible for templates without any properties (solutions welcome in PRs!).
    template: TemplateFn<Props, G>,
    /// A function that gets the paths to render for at built-time. This is equivalent to `get_static_paths` in NextJS. If
    /// `incremental_path_rendering` is `true`, more paths can be rendered at request time on top of these.
    get_build_paths: Option<GetBuildPathsFn>,
    /// Defines whether or not any new paths that match this template will be prerendered and cached in production. This allows you to
    /// have potentially billions of templates and retain a super-fast build process. The first user will have an ever-so-slightly slower
    /// experience, and everyone else gets the beneftis afterwards. This requires `get_build_paths`. Note that the template root will NOT
    /// be rendered on demand, and must be explicitly defined if it's wanted. It can uuse a different template.
    incremental_path_rendering: bool,
    /// A function that gets the initial state to use to prerender the template at build time. This will be passed the path of the template, and
    /// will be run for any sub-paths. This is equivalent to `get_static_props` in NextJS.
    get_build_state: Option<GetBuildStateFn<Props>>,
    /// A function that will run on every request to generate a state for that request. This allows server-side-rendering. This is equivalent
    /// to `get_server_side_props` in NextJS. This can be used with `get_build_state`, though custom amalgamation logic must be provided.
    // TODO add request data to be passed in here
    get_request_state: Option<GetRequestStateFn<Props>>,
    /// A function to be run on every request to check if a template prerendered at build-time should be prerendered again. This is equivalent
    /// to incremental static rendering (ISR) in NextJS. If used with `revalidate_after`, this function will only be run after that time
    /// period. This function will not be parsed anything specific to the request that invoked it.
    should_revalidate: Option<ShouldRevalidateFn>,
    /// A length of time after which to prerender the template again. This is equivalent to ISR in NextJS.
    revalidate_after: Option<String>,
}
impl<Props: Serialize + DeserializeOwned, G: GenericNode> Template<Props, G> {
    /// Creates a new template definition.
    pub fn new(path: impl Into<String> + std::fmt::Display) -> Self {
        Self {
            path: path.to_string(),
            // We only need the `Props` generic here
            template: Box::new(|_: Option<Props>| sycamore::template! {}),
            get_build_paths: None,
            incremental_path_rendering: false,
            get_build_state: None,
            get_request_state: None,
            should_revalidate: None,
            revalidate_after: None,
        }
    }

    // Render executors
    /// Executes the user-given function that renders the template on the server-side (build or request time).
    pub fn render_for_template(&self, props: Option<Props>) -> SycamoreTemplate<G> {
        (self.template)(props)
    }
    /// Gets the list of templates that should be prerendered for at build-time.
    pub fn get_build_paths(&self) -> Result<Vec<String>> {
        if let Some(get_build_paths) = &self.get_build_paths {
            // TODO support error handling for render functions
            Ok(get_build_paths())
        } else {
            bail!(ErrorKind::TemplateFeatureNotEnabled(self.path.clone(), "build_paths".to_string()))
        }
    }
    /// Gets the initial state for a template. This needs to be passed the full path of the template, which may be one of those generated by
    /// `.get_build_paths()`.
    pub fn get_build_state(&self, path: String) -> Result<Props> {
        if let Some(get_build_state) = &self.get_build_state {
            // TODO support error handling for render functions
            Ok(get_build_state(path))
        } else {
            bail!(ErrorKind::TemplateFeatureNotEnabled(self.path.clone(), "build_state".to_string()))
        }
    }

    // Value getters
    /// Gets the path of the template. This is the root path under which any generated pages will be served. In the simplest case, there will
    /// only be one page rendered, and it will occupy that root position.
    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    // Render characteristic checkers
    /// Checks if this template can revalidate existing prerendered templates.
    pub fn revalidates(&self) -> bool {
        self.should_revalidate.is_some() || self.revalidate_after.is_some()
    }
    /// Checks if this template can render more templates beyond those paths it explicitly defines.
    pub fn uses_incremental(&self) -> bool {
        self.incremental_path_rendering
    }
    /// Checks if this template is a template to generate paths beneath it.
    pub fn uses_build_paths(&self) -> bool {
        self.get_build_paths.is_some()
    }
    /// Checks if this template needs to do anything on requests for it.
    pub fn uses_request_state(&self) -> bool {
        self.get_request_state.is_some()
    }
    /// Checks if this template needs to do anything at build time.
    pub fn uses_build_state(&self) -> bool {
        self.get_build_state.is_some()
    }
    /// Checks if this template defines no rendering logic whatsoever. Such templates will be rendered using SSG.
    pub fn is_basic(&self) -> bool {
        !self.uses_build_paths() &&
        !self.uses_build_state() &&
        !self.uses_request_state() &&
        !self.revalidates() &&
        !self.uses_incremental()
    }

    // Builder setters
    pub fn template(mut self, val: TemplateFn<Props, G>) -> Template<Props, G> {
        self.template = val;
        self
    }
    pub fn build_paths_fn(mut self, val: GetBuildPathsFn) -> Template<Props, G> {
        self.get_build_paths = Some(val);
        self
    }
    pub fn incremental_path_rendering(mut self, val: bool) -> Template<Props, G> {
        self.incremental_path_rendering = val;
        self
    }
    pub fn build_state_fn(mut self, val: GetBuildStateFn<Props>) -> Template<Props, G> {
        self.get_build_state = Some(val);
        self
    }
    pub fn request_state_fn(mut self, val: GetRequestStateFn<Props>) -> Template<Props, G> {
        self.get_request_state = Some(val);
        self
    }
    pub fn should_revalidate(mut self, val: ShouldRevalidateFn) -> Template<Props, G> {
        self.should_revalidate = Some(val);
        self
    }
    pub fn revalidate_after(mut self, val: String) -> Template<Props, G> {
        self.revalidate_after = Some(val);
        self
    }
}
