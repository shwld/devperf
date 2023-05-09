use super::schema::*;

pub fn perform(token: UnvalidatedHerokuAppName) -> Result<ValidateHerokuAppNameEvent, ValidateHerokuAppNameError> {
    ValidatedHerokuAppName::new(token)
}

// PRIVATE

impl ValidatedHerokuAppName {
    pub fn new(token: Option<String>) -> Result<Self, ValidateHerokuAppNameError> {
      if let Some(token) = token {
        if token.len() > 1 {
          Ok(ValidatedHerokuAppName(token.to_string()))
        } else {
          Err(ValidateHerokuAppNameError::InvalidToken("GitHub personal token is invalid".to_string()))
        }
      } else {
        Err(ValidateHerokuAppNameError::Required("GitHub personal token is empty".to_string()))
      }
    }

    pub fn to_string(self) -> String {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::project_creating::validate_heroku_app_name::schema::ValidateHerokuAppName;

    #[test]
    fn verify_perform_type() {
        // 型チェックのために代入する
        let _type_check: ValidateHerokuAppName = super::perform;
    }
}
