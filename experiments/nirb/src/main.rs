use clap::{App,Arg};
use std::process::Command;
use std::env;
use std::str;

#[derive(Debug)]
struct Export {
    var: String,
    value: String
}

#[allow(unused_variables)]
impl Export {
    fn new(var: &str, value: &str) -> Export {
        Export {
            var: String::from(var),
            value: String::from(value)
        }
    }

    fn set_env(self: &Export) -> Result<String, String> {
        let env_name = self.var.clone();
        env::set_var(&env_name, self.value.clone());
        Ok(env_name)
    }

    // fn printenvs(self: &Export) {
    //     Command::new("printenv")
    //         .spawn()
    //         .expect("printenv command failed to start");
    // }

    fn set_op_pass(self: &Export) -> Result<Vec<u8>, String> {
        let output = Command::new("op")
            .args(["item", "get", "test-command", "--field=password"])
            .output()
            .expect("op command failed to start");
        Ok(output.stdout)
    }

  //  fn get_vault_token() {
  //              VAULT_TOKEN=$(curl -d "{\"password\": \"$(op read op://Private/ldap/password)\"}" "$VAULT_ADDR/v1/auth/ldap/login/svaz" -H "X-Vault-Namespace: $VAULT_NAMESPACE" -s | jq '.auth.client_token' | tr -d '"')

  //  }

    fn print_one_env(
        self: &Export,
        var: &str
        ) {
        let env_var_err_message = format!("${} is not set", &var);
        let v = env::var(var).expect(&env_var_err_message);
        println!("env {} is set to: {}", &var, v)
    }
}


fn main() {
    let args = App::new("nomad env on ruby irb")
     .version("0.1")
     .about("login on nomad envs with a ruby repl")
     .arg(Arg::with_name("env")
       .help("The env to login to")
       .takes_value(true)
       .required(true))
     .get_matches();

   let env = args.value_of("env").unwrap();
   let namespace = &env;
   let cluster = "cluster";
   println!("env: {}, namespace: {}, cluster: {}", &env, &namespace, &cluster);

   // vault
   let vault_env_names = &["VAULT_NAMESPACE", "VAULT_ADDR"];
   let vault_namespace = format!("{}-my-env", &env);
   let vault_addr = format!("https://vault.{}.{}.addr:1119", &namespace, &cluster);
   let vault_env_values = &[&vault_namespace, &vault_addr];

   let export_vault = Export::new("VAULT_NAMESPACE", "dev");
   let op_pass = export_vault.set_op_pass().unwrap();
   let s = match str::from_utf8(&op_pass) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

   println!("op_pass is {:?}", s);

   for (var_name, var_value) in vault_env_names.iter().zip(vault_env_values) {
       let export_vault = Export::new(&var_name, &var_value);
       let var_name_set = export_vault.set_env().unwrap();
       export_vault.print_one_env(&var_name_set);
   };

   // nomad
   let nomad_env_names: Vec<&str> = vec!["NOMAD_NAMESPACE", "NOMAD_ADDR"];
   let nomad_namespace = format!("{}-my-env", &env);
   let nomad_addr = format!("https://nomad.{}.{}.addr:1119", &namespace, &cluster);
   let nomad_env_values: Vec<&str> = vec![&nomad_namespace, &nomad_addr];

   for (var_name, var_value) in nomad_env_names.iter().zip(nomad_env_values) {
       let export_nomad = Export::new(&var_name, &var_value);
       let var_name_set = export_nomad.set_env().unwrap();
       export_nomad.print_one_env(&var_name_set);
   };

   // consul
   let consul_env_names: Vec<&str> = vec!["CONSUL_HTTP_ADDR"];
   let consul_addr = format!("https://consul.{}.{}.addr:1119", &namespace, &cluster);
   let consul_env_values: Vec<&str> = vec![&consul_addr];

   for (var_name, var_value) in consul_env_names.iter().zip(consul_env_values) {
       let export_consul = Export::new(&var_name, &var_value);
       let var_name_set = export_consul.set_env().unwrap();
       export_consul.print_one_env(&var_name_set);
   };
}

