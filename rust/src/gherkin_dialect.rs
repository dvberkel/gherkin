use std::collections::HashMap;
use std::fs::File;
use serde_json;
use ast::Location;
use {ParserError, ErrorKind};
use serde::Deserialize;
use serde::Deserializer;
use serde::de::{Visitor, MapVisitor, Error};

#[derive(Debug, PartialEq, Clone)]
pub struct LanguageSettings {
    and: Vec<String>,
    background: Vec<String>,
    but: Vec<String>,
    examples: Vec<String>,
    feature: Vec<String>,
    given: Vec<String>,
    scenario: Vec<String>,
    scenario_outline: Vec<String>,
    then: Vec<String>,
    when: Vec<String>,
    name: String,
    native: String
}

enum Keywords {
    And,
    Background,
    But,
    Examples,
    Feature,
    Given,
    Scenario,
    ScenarioOutline,
    Then,
    When,
    Name,
    Native
}

impl Deserialize for Keywords {
    fn deserialize<D>(deserializer: &mut D) -> Result<Keywords, D::Error>
        where D: Deserializer
    {
        struct KeywordsVisitor;

        impl Visitor for KeywordsVisitor {
            type Value = Keywords;

            fn visit_str<E>(&mut self, value: &str) -> Result<Keywords, E>
                where E: Error
            {
                match value {
                    "and" => Ok(Keywords::And),
                    "background" => Ok(Keywords::Background),
                    "but" => Ok(Keywords::But),
                    "examples" => Ok(Keywords::Examples),
                    "feature" => Ok(Keywords::Feature),
                    "given" => Ok(Keywords::Given),
                    "scenario" => Ok(Keywords::Scenario),
                    "scenarioOutline" => Ok(Keywords::ScenarioOutline),
                    "then" => Ok(Keywords::Then),
                    "when" => Ok(Keywords::When),
                    "name" => Ok(Keywords::Name),
                    "native" => Ok(Keywords::Native),
                    _ => Err(Error::custom("expected valid keyword")),
                }
            }
        }

        deserializer.deserialize(KeywordsVisitor)
    }
}

impl Deserialize for LanguageSettings {
    fn deserialize<D>(deserializer: &mut D) -> Result<LanguageSettings, D::Error>
        where D: Deserializer
    {
        static FIELDS: &'static [&'static str] = &["x", "y"];
        deserializer.deserialize_struct("LanguageSettings", FIELDS, LanguageSettingsVisitor)
    }
}

struct LanguageSettingsVisitor;

impl Visitor for LanguageSettingsVisitor {
    type Value = LanguageSettings;

    fn visit_map<V>(&mut self, mut visitor: V) -> Result<LanguageSettings, V::Error>
    where V: MapVisitor {
        let mut and = None;
        let mut background = None;
        let mut but = None;
        let mut examples = None;
        let mut feature = None;
        let mut given = None;
        let mut scenario = None;
        let mut scenario_outline = None;
        let mut then = None;
        let mut when = None;
        let mut name = None;
        let mut native = None;
        loop {
            match try!(visitor.visit_key()) {
                Some(Keywords::And) => and = Some(try!(visitor.visit_value())),
                Some(Keywords::Background) => background = Some(try!(visitor.visit_value())),
                Some(Keywords::But) => but = Some(try!(visitor.visit_value())),
                Some(Keywords::Examples) => examples = Some(try!(visitor.visit_value())),
                Some(Keywords::Feature) => feature = Some(try!(visitor.visit_value())),
                Some(Keywords::Given) => given = Some(try!(visitor.visit_value())),
                Some(Keywords::Scenario) => scenario = Some(try!(visitor.visit_value())),
                Some(Keywords::ScenarioOutline) => scenario_outline = Some(try!(visitor.visit_value())),
                Some(Keywords::Then) => then = Some(try!(visitor.visit_value())),
                Some(Keywords::When) => when = Some(try!(visitor.visit_value())),
                Some(Keywords::Name) => name = Some(try!(visitor.visit_value())),
                Some(Keywords::Native) => native = Some(try!(visitor.visit_value())),
                None => break
            }
        }
        let and = match and {
            Some(and) => and,
            None => try!(visitor.missing_field("and"))
        };
        let background = match background {
            Some(background) => background,
            None => try!(visitor.missing_field("background"))
        };
        let but = match but {
            Some(but) => but,
            None => try!(visitor.missing_field("but"))
        };
        let examples = match examples {
            Some(examples) => examples,
            None => try!(visitor.missing_field("examples"))
        };
        let feature = match feature {
            Some(feature) => feature,
            None => try!(visitor.missing_field("feature"))
        };
        let given = match given {
            Some(given) => given,
            None => try!(visitor.missing_field("given"))
        };
        let scenario = match scenario {
            Some(scenario) => scenario,
            None => try!(visitor.missing_field("scenario"))
        };
        let scenario_outline = match scenario_outline {
            Some(scenario_outline) => scenario_outline,
            None => try!(visitor.missing_field("scenarioOutline"))
        };
        let then = match then {
            Some(then) => then,
            None => try!(visitor.missing_field("then"))
        };
        let when = match when {
            Some(when) => when,
            None => try!(visitor.missing_field("when"))
        };
        let name = match name {
            Some(name) => name,
            None => try!(visitor.missing_field("name"))
        };
        let native = match native {
            Some(native) => native,
            None => try!(visitor.missing_field("native"))
        };

        try!(visitor.end());
        Ok(LanguageSettings{
            and: and,
            background: background,
            but: but,
            examples: examples,
            feature: feature,
            given: given,
            scenario: scenario,
            scenario_outline: scenario_outline,
            then: then,
            when: when,
            name: name,
            native: native
        })
    }
}

pub struct GherkinDialectProvider {
    default: String,
    dialects: HashMap<String, GherkinDialect>
}

impl GherkinDialectProvider {
    pub fn new_with_default(default: &str) -> Result<GherkinDialectProvider, ParserError> {
        let settings_file = match File::open("resources/gherkin-languages.json") {
            Ok(settings_file) => settings_file,
            Err(_) => return Err(ParserError::new(ErrorKind::ResourceNotFound("resources/gherkin-languages.json".to_string()), Location::new(1, 1)))
        };
        let language_settings: HashMap<String, LanguageSettings> = match serde_json::from_reader(settings_file){
            Ok(language_settings) => language_settings,
            Err(json_error) => return Err(ParserError::new(ErrorKind::MalformedResource(json_error.to_string()), Location::new(1, 1)))
        };
        let mut dialects = HashMap::new();
        for (language, language_setting) in language_settings.iter() {
            dialects.insert(language.to_string(), GherkinDialect::new(language.to_string(), language_setting.clone()));
        }
        Ok(GherkinDialectProvider {
            default: default.to_string(),
            dialects: dialects
        })
    }

    pub fn new() -> Result<GherkinDialectProvider, ParserError> {
        GherkinDialectProvider::new_with_default("en")
    }

    pub fn get_dialect<'a>(&'a self, language: &str, location: Location) -> Result<&'a GherkinDialect, ParserError> {
        match self.dialects.get(language) {
            Some(dialect) => Ok(dialect),
            None => Err(ParserError::new(ErrorKind::NoSuchLanguage(language.to_string()), location))
        }
    }

    pub fn get_default<'a>(&'a self, location: Location) -> Result<&'a GherkinDialect, ParserError> {
        self.get_dialect(&self.default, location)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct GherkinDialect {
    language: String,
    feature_keywords: Vec<String>,
    background_keywords: Vec<String>,
    scenario_keywords: Vec<String>,
    scenario_outline_keywords: Vec<String>,
    examples_keywords: Vec<String>,
    given_step_keywords: Vec<String>,
    when_step_keywords: Vec<String>,
    then_step_keywords: Vec<String>,
    and_step_keywords: Vec<String>,
    but_step_keywords: Vec<String>,
    step_keywords: Vec<String>
}



impl GherkinDialect {
    fn new(language: String, language_setting: LanguageSettings) -> GherkinDialect {
        let mut step_keywords = language_setting.given.clone();
        step_keywords.extend(language_setting.when.clone());
        step_keywords.extend(language_setting.then.clone());
        step_keywords.extend(language_setting.and.clone());
        step_keywords.extend(language_setting.but.clone());
        GherkinDialect {
            language: language,
            feature_keywords: language_setting.feature,
            background_keywords: language_setting.background,
            scenario_keywords: language_setting.scenario,
            scenario_outline_keywords: language_setting.scenario_outline,
            examples_keywords: language_setting.examples,
            given_step_keywords: language_setting.given,
            when_step_keywords: language_setting.when,
            then_step_keywords: language_setting.then,
            and_step_keywords: language_setting.and,
            but_step_keywords: language_setting.but,
            step_keywords: step_keywords
        }
    }



    pub fn get_language(&self) -> String {
        self.language.clone()
    }

    pub fn get_feature_keywords(&self) -> &Vec<String> {
        &self.feature_keywords
    }

    pub fn get_background_keywords(&self) -> &Vec<String> {
        &self.background_keywords
    }

    pub fn get_scenario_keywords(&self) -> &Vec<String> {
        &self.scenario_keywords
    }

    pub fn get_scenario_outline_keywords(&self) -> &Vec<String> {
        &self.scenario_outline_keywords
    }

    pub fn get_examples_keywords(&self) -> &Vec<String> {
        &self.examples_keywords
    }

    pub fn get_step_keywords(&self) -> &Vec<String> {
        &self.step_keywords
    }

}



#[cfg(test)]
mod test {
    use super::GherkinDialectProvider;
    use ast::Location;
    

    #[test]
    fn create_dialect() {
        let factory = GherkinDialectProvider::new().unwrap();
        assert!(factory.get_dialect("es", Location::new(1, 5)).is_ok());

        let factory = GherkinDialectProvider::new_with_default("ru").unwrap();
        assert_eq!(factory.get_dialect("ru", Location::new(6, 7)), factory.get_default(Location::new(43, 8)))
    }

}
