use fluent_bundle::bundle::FluentBundle;
use fluent_bundle::{FluentArgs, FluentResource};
use include_dir::{include_dir, Dir};
use intl_memoizer::concurrent::IntlLangMemoizer;
use std::collections::HashMap;
use std::sync::RwLock;
use unic_langid::LanguageIdentifier;

const PROJECT_DIR: Dir = include_dir!("../resources/locales");

lazy_static! {
  pub static ref I18N: RwLock<I18n> = RwLock::new(I18n::new("zh-Hans"));
}

// Contains all gettext catalogs we use in compiled form.
pub struct I18n {
  // pub catalogs: Vec<(&'static str, Catalog)>,
  fallback_lang: String,
  bundle_map: HashMap<String, FluentBundle<FluentResource, IntlLangMemoizer>>,
}
impl I18n {
  pub fn new(fallback_language: &str) -> I18n {
    let mut bundle_map: HashMap<String, FluentBundle<FluentResource, IntlLangMemoizer>> =
      HashMap::new();
    let fallback_language_str: String = fallback_language.to_string();
    let fallback_lang_id: LanguageIdentifier = fallback_language_str
      .parse()
      .expect("Parsing default lang failed.");

    let dirs = PROJECT_DIR.dirs();
    for locale_dir in dirs {
      let lang = locale_dir.path().to_str().unwrap();
      let lang_identifier: LanguageIdentifier = lang.parse().expect("Parsing lang folder failed.");
      let mut bundle: FluentBundle<FluentResource, IntlLangMemoizer> =
        FluentBundle::new_concurrent(vec![lang_identifier, fallback_lang_id.clone()]);
      bundle.set_use_isolating(false);
      let files = locale_dir.files();
      for locale_file in files {
        let locale_file_path = locale_file.path();
        let locale_file_content = locale_file.contents_utf8().unwrap();

        let res = FluentResource::try_new(locale_file_content.to_string()).expect(
          format!(
            "Failed to parse an FTL string at {}",
            locale_file_path.to_str().unwrap()
          )
          .as_ref(),
        );
        bundle
          .add_resource(res)
          .expect("Failed to add FTL resources to the bundle.");
      }
      bundle_map.insert(lang.to_string(), bundle);
    }
    I18n {
      fallback_lang: fallback_language.to_string(),
      bundle_map: bundle_map,
    }
  }

  pub fn with_args(&self, id: &str, lang: &str, args: FluentArgs) -> String {
    let bundle = self.get_bundle_by_lang(lang);
    let msg = bundle.get_message(id);

    if let Some(the_msg) = msg {
      // 5. Format the value of the message
      let mut errors = vec![];
      let pattern = the_msg.value().expect("Message has no value.");
      let x = bundle.format_pattern(&pattern, Some(&args), &mut errors);
      return x.to_string();
    } else {
      // get fallback lang

      if lang == self.fallback_lang {
        return id.to_string();
      } else {
        return self.with_args(id, &self.fallback_lang, args);
      }
    }
  }
  pub fn with_lang(&self, id: &str, lang: &str) -> String {
    let bundle = self.get_bundle_by_lang(lang);
    let msg = bundle.get_message(id);
    if let Some(the_msg) = msg {
      let mut errors = vec![];
      let pattern = the_msg.value().expect("Message has no value.");
      let value = bundle.format_pattern(&pattern, None, &mut errors);
      return value.to_string();
    } else {
      if lang == self.fallback_lang {
        return id.to_string();
      } else {
        return self.with_lang(id, &self.fallback_lang);
      }
    }
  }
  pub fn get_bundle_by_lang(&self, lang: &str) -> &FluentBundle<FluentResource, IntlLangMemoizer> {
    let bundle_option = self.bundle_map.get(lang);
    let bundle: &FluentBundle<FluentResource, IntlLangMemoizer>;
    if let Some(the_bundle) = bundle_option {
      bundle = the_bundle;
    } else {
      bundle = self
        .bundle_map
        .get::<str>(&self.fallback_lang)
        .expect("unwrap fallback bundle map failed");
    }
    return bundle;
  }
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn new_i18n() {
    let i18n = I18n::new("en-US");
    let message2 = i18n.with_lang("hello", "zh-Hans");
    let mut args = FluentArgs::new();
    args.set("random", "test");
    let message3 = i18n.with_args("default-name", "zh-Hans", args);

    assert_eq!(message2, "你好");
    assert_eq!(message3, "用户test")
  }
}