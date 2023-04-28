# zelis-test-rs

A program that will read JSON either from STDIN or a file and replaces any semicolons(";") with colons(":"). Large JSON files is an expected output so processing is done in parallel.

### Usage

##### Argument
Passing a filepath as an argument
```bash
$ cargo run data/test1.json
{
    "reporting_entity_name": "Zelis Test",
    "reporting_entity_type": "health insurance issuer",
    "reporting_plans": [{
      "plan_name": "1gb Logging",
      "plan_id_type": "ein",
      "plan_id": "17090731852489",
      "plan_market_type": "individual"
    }],
    "last_updated_on": "2022-02-15",
    "version": "1.0.0",
    "in_network": [{ 
    "negotiation_arrangement": "ffs",
    "name": "Fake Procedure Injgzmtk0mdm",
    "billing_code_type": "CPT",
    "billing_code_type_version": "2020",
    "billing_code": "53310",
    "description": "Description of Fake Procedure Injgzmtk0mdm"
}
```

##### Passing a filepath to read from via STDIN
```bash
cargo run                                                      
data/test1.json
{
    "reporting_entity_name": "Zelis Test",
    "reporting_entity_type": "health insurance issuer",
    "reporting_plans": [{
      "plan_name": "1gb Logging",
      "plan_id_type": "ein",
      "plan_id": "17090731852489",
      "plan_market_type": "individual"
    }],
    "last_updated_on": "2022-02-15",
    "version": "1.0.0",
    "in_network": [{ 
    "negotiation_arrangement": "ffs",
    "name": "Fake Procedure Injgzmtk0mdm",
    "billing_code_type": "CPT",
    "billing_code_type_version": "2020",
    "billing_code": "53310",
    "description": "Description of Fake Procedure Injgzmtk0mdm"
}
```

##### Piping input from a file
```bash
$ cat data/test1.json | cargo run                      
{
    "reporting_entity_name": "Zelis Test",
    "reporting_entity_type": "health insurance issuer",
    "reporting_plans": [{
      "plan_name": "1gb Logging",
      "plan_id_type": "ein",
      "plan_id": "17090731852489",
      "plan_market_type": "individual"
    }],
    "last_updated_on": "2022-02-15",
    "version": "1.0.0",
    "in_network": [{ 
    "negotiation_arrangement": "ffs",
    "name": "Fake Procedure Injgzmtk0mdm",
    "billing_code_type": "CPT",
    "billing_code_type_version": "2020",
    "billing_code": "53310",
    "description": "Description of Fake Procedure Injgzmtk0mdm"
}
```

##### Specifying input file via `--input`
```bash
cargo run -- --input data/test1.json
{
    "reporting_entity_name": "Zelis Test",
    "reporting_entity_type": "health insurance issuer",
    "reporting_plans": [{
      "plan_name": "1gb Logging",
      "plan_id_type": "ein",
      "plan_id": "17090731852489",
      "plan_market_type": "individual"
    }],
    "last_updated_on": "2022-02-15",
    "version": "1.0.0",
    "in_network": [{ 
    "negotiation_arrangement": "ffs",
    "name": "Fake Procedure Injgzmtk0mdm",
    "billing_code_type": "CPT",
    "billing_code_type_version": "2020",
    "billing_code": "53310",
    "description": "Description of Fake Procedure Injgzmtk0mdm"
}
```

##### Specifying output file to write to
```bash
cargo run -- --input data/test1.json --output data/result1.json
Successfully wrote results to file
```
Contents of `data/result1.json`
```
{
    "reporting_entity_name": "Zelis Test",
    "reporting_entity_type": "health insurance issuer",
    "reporting_plans": [{
      "plan_name": "1gb Logging",
      "plan_id_type": "ein",
      "plan_id": "17090731852489",
      "plan_market_type": "individual"
    }],
    "last_updated_on": "2022-02-15",
    "version": "1.0.0",
    "in_network": [{ 
    "negotiation_arrangement": "ffs",
    "name": "Fake Procedure Injgzmtk0mdm",
    "billing_code_type": "CPT",
    "billing_code_type_version": "2020",
    "billing_code": "53310",
    "description": "Description of Fake Procedure Injgzmtk0mdm"
}
```
