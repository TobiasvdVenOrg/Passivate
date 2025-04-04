use crate::passivate_grcov::parse_covdir;


#[test]
pub fn parse() {
    let json = r#"{
   "children":{
      "src":{
         "children":{
            "lib.rs":{
               "coverage":[
                  -1,
                  4,
                  4,
                  4,
                  -1,
                  2,
                  2,
                  2
               ],
               "coveragePercent":100.0,
               "linesCovered":6,
               "linesMissed":0,
               "linesTotal":6,
               "name":"lib.rs"
            }
         },
         "coveragePercent":100.0,
         "linesCovered":6,
         "linesMissed":0,
         "linesTotal":6,
         "name":"src"
      },
      "tests":{
         "children":{
            "add_tests.rs":{
               "coverage":[
                  -1,
                  -1,
                  -1,
                  1,
                  1,
                  1,
                  1,
                  -1,
                  -1,
                  1,
                  1,
                  1,
                  1
               ],
               "coveragePercent":100.0,
               "linesCovered":8,
               "linesMissed":0,
               "linesTotal":8,
               "name":"add_tests.rs"
            },
            "multiply_tests.rs":{
               "coverage":[
                  -1,
                  -1,
                  -1,
                  1,
                  1,
                  1,
                  1
               ],
               "coveragePercent":100.0,
               "linesCovered":4,
               "linesMissed":0,
               "linesTotal":4,
               "name":"multiply_tests.rs"
            }
         },
         "coveragePercent":100.0,
         "linesCovered":12,
         "linesMissed":0,
         "linesTotal":12,
         "name":"tests"
      }
   },
   "coveragePercent":100.0,
   "linesCovered":18,
   "linesMissed":0,
   "linesTotal":18,
   "name":""
}"#;

    let result = parse_covdir(json).unwrap();

    assert_eq!(100.0, result.coverage_percent);
}