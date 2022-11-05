use flows_connector_dsi::{github::inbound, sendgrid::outbound};
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

pub fn _run(s: String) -> Result<String, String> {
    let payload = inbound(s)?;
    let email = match payload
        .starred_at
        .as_ref()
        .and(payload.sender.email.as_ref())
    {
        Some(e) => e,
        None => return Ok(String::new()),
    };

    let sender = &payload.sender.login;
    let repo = &payload.get_repository()?.full_name;

    if payload.get_action()? == "created" {
        outbound(vec![email])
            .subject(" 😉 Thank you for your star!")
            .content(format!(
                r#"
                
Welcome to the {} community! Here are some resources to get you started. Please feel free to reach out to us on GitHub or Discord if you have questions or encounter any issues. Happy hacking!<br/>
<br/>
Please refer to our quick start guides to install WasmEdge and run your Wasm apps:<br/>
https://wasmedge.org/book/en/quick_start.html
<br/><br/>
The repo below contains a complete demo app in WasmEdge. It is a microservice with a web frontend and a MySQL database backend.<br/>
https://github.com/second-state/microservice-rust-mysql
<br/><br/>
WasmEdge has collaborated with Docker to create a seamless developer experience for building, sharing, and running applications with mixed Linux containers and Wasm sandboxes. You can read Docker's announcement below, and try the above microservice demo in Docker Desktop or Docker CLI!<br/>
https://www.docker.com/blog/docker-wasm-technical-preview/               
<br/><br/>
Finally, If you are in Detorit for KubeCon this week, please come by our kiosk in the CNCF projects area and say hi!
<br/><br/>

Look forward to your contributions,<br/>
Maintainers at repository {}"#,
                sender, repo, repo
            ))
            .build()
    } else {
        outbound(vec![email])
            .subject(" 😿 Sorry to lose you")
            .content(format!(
                r#"
Hi {},<br/>

Sorry to see you go! We value your feedback and suggestions. Please do let us know how we might improve the repository {} (just reply to this email). We wish to see your around in the community!<br/>

Best Regards,<br/>
Maintainers at repository {}"#,
                sender, repo, repo
            ))
            .build()
    }
}
