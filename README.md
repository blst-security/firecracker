
<div  align="center">

![cherry_bomb_v1.0](https://raw.githubusercontent.com/blst-security/cherrybomb/main/images/cherrybomb_github_art_v2-1%20(1).png)

  

<h1>Stop half-done API specifications</h1>

[![Maintained by blst security](https://img.shields.io/badge/maintained%20by-blst%20security-4F46E5)]()

[![docs](https://img.shields.io/badge/docs-passing-brightgreen)]()

[![Discord Shield](https://discordapp.com/api/guilds/914846937327497307/widget.png?style=shield)](https://discord.gg/WdHhv4DqwU)

</div>

  

# 💣 What is Cherrybomb?

Cherrybomb is an CLI tool written  in Rust that helps prevent incorrect code implementation early in development. It works by validating and testing your API using an OpenAPI file. Its main goal is to reduce security errors and ensure your API functions as intended.

  
  

# 🔨 How does it work?


Cherrybomb makes sure your API is working correctly. It checks your API's spec file (OpenAPI Specification) for good practices and makes sure it follows the OAS rules. Then, it tests your API for common issues and vulnerabilities. If any problems are found, Cherrybomb gives you a detailed report with the exact location of the problem so you can fix it easily.

  

# 🐾 Get Started

## Installation



##### Linux/MacOS:

```

DEPRECATED FOR NOW
```

The script requires sudo permissions to move the cherrybomb bin into <b>/usr/local/bin/</b>.</br>

(If you want to view the shell script(or even help to improving it - [/scripts/install.sh](/scripts/install.sh))

 ##### Containerized version
 You can get Cherrybomb through its containerized version which is hosted on AWS ECR, and requires an API key that you can get on that address(the loading is a bit slow) - DEPRECATED FOR NOW

```
docker run --mount type=bind,source=[PATH TO OAS],destination=/home public.ecr.aws/blst-security/cherrybomb:latest cherrybomb -f /home/[OAS NAME] --api-key=[API-KEY]
```

#### Get it from crates.io

```bash

cargo install cherrybomb

```

If you don't have cargo installed, you can install it from [here](https://doc.rust-lang.org/cargo/getting-started/installation.html)



#### Building from Sources

You can also build Cherrybomb from sources by cloning this repo, and building it using cargo.

```

git clone https://github.com/blst-security/cherrybomb && cd cherrybomb

```
The main branch's Cargo.toml file uses `cherrybomb-engine` and `cherrybomb-oas` from crates.io. 

if you want build those from source too, you can change the following files:

(remove the version number and replace with the path to the local repo)



```
cherrybomb/Cargo.toml:
cherrybomb-engine = version => { path = "cherrybomb-engine" }
```
 
```
cherrybomb/cherrybomb-engine/Cargo.toml:
cherrybomb-oas = version => { path = "../cherrybomb-oas" }
```

```
cargo build --release
sudo mv ./target/release/cherrybomb /usr/local/bin # or any other directory in your PATH
```
  

  
### Profile 
 
Profiles allow you to choose the type of check you want to use.
```
- info: only generates param and endpoint tables
- normal:  both active and passive
- intrusive: active and intrusive [in development]
- passive: only passive tests
- full: all the options
```

### Config 



With a configuration file, you can easily edit, view, Cherrybomb's options.
The config file allows you to set the running profile, location of the oas file, the verbosity and ignore the TLS error.

Config also allows you to override the server's URL with an array of servers, and add security to the request [in development]. 

Notice that CLI arguments parameter will override config options if both are set.

You can also add or remove checks from a profile using `passive/active-include/exclude`. [in development]

```
cherrybomb --config  <CONFIG_FILE>
```


Structure of config file:
```
{
"file" : "open-api.json",
"verbosity" : "Normal", 
"profile" :   "Normal",
"passive_include" : ["check1, checks2"],
"active_include": ["check3, check4"],
"servers_override" , ["http://server/"],
"security":  [{
    "auth_type": "Basic",
    "auth_value" : token_value,
    "auth_scope" : scope_name
    }],
"ignore_tls_errors" : true, 
"no_color" : false,
}
```



# Usage

After installing, verify it's working by running

```
cherrybomb --version

```

### OpenAPI specification


``` cherrybomb --file <PATH> --profile passive ```

Passive Output example:

![passive_output](https://raw.githubusercontent.com/blst-security/cherrybomb/main/images/passive1_0.png)


### Generate Info Table


```
cherrybomb --file <PATH> --profile info

```
Parameter table output:

  ![parameter_output](https://raw.githubusercontent.com/blst-security/cherrybomb/main/images/param_v1.png)

Endpoint table output:

  ![endpoint_output](https://raw.githubusercontent.com/blst-security/cherrybomb/main/images/endpoint_v1.png)




# 🍻 Integration

DEPRECATED FOR NOW - WILL BE REPLACED SOON

You can embed it into your CI pipeline, and If you plan on doing that I would recommend that you go to our [website](), sign up, go through the [CI pipeline integration wizard](), and copy the groovy/GitHub actions snippet built for you.

</br>Example:

![CI pipeline builder output](https://raw.githubusercontent.com/blst-security/cherrybomb/main/images/ci_output.png)

# 💪 Support

  

### Get help

If you have any questions you can ask us on our [discord server](https://discord.gg/WdHhv4DqwU).


You are also welcome to open an Issue here on GitHub.
