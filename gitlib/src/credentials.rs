#[derive(Debug)]
pub enum CredentialType {
    UserPassPlaintext,
    SshKey,
    SshMemory,
    SshCustom,
    Default,
    SshInteractive,
    Username,
    Unknown,
}

impl From<git2::CredentialType> for CredentialType {
    fn from(cred: git2::CredentialType) -> Self {
        match cred {
            git2::CredentialType::USER_PASS_PLAINTEXT => CredentialType::UserPassPlaintext,
            git2::CredentialType::SSH_KEY => CredentialType::SshKey,
            git2::CredentialType::SSH_MEMORY => CredentialType::SshMemory,
            git2::CredentialType::SSH_CUSTOM => CredentialType::SshCustom,
            git2::CredentialType::DEFAULT => CredentialType::Default,
            git2::CredentialType::SSH_INTERACTIVE => CredentialType::SshInteractive,
            git2::CredentialType::USERNAME => CredentialType::Username,
            _ => CredentialType::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct Credentials {
    cred: git2::CredentialType,
    multiple_creds: bool,
    index: i8,
}

impl From<git2::CredentialType> for Credentials {
    fn from(cred: git2::CredentialType) -> Self {
        Self {
            cred,
            multiple_creds: cred.bits().count_ones() > 1,
            index: -1,
        }
    }
}

impl Iterator for Credentials {
    type Item = CredentialType;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;

        // Optimization for single credential
        if !self.multiple_creds {
            if self.index == 0 {
                return Some(CredentialType::from(self.cred));
            } else {
                return None;
            }
        }

        const CRED_TYPE: [git2::CredentialType; 7] = [
            git2::CredentialType::USER_PASS_PLAINTEXT,
            git2::CredentialType::SSH_KEY,
            git2::CredentialType::SSH_MEMORY,
            git2::CredentialType::SSH_CUSTOM,
            git2::CredentialType::DEFAULT,
            git2::CredentialType::SSH_INTERACTIVE,
            git2::CredentialType::USERNAME,
        ];

        // Only make it here if there are multiple credentials
        loop {
            if self.index as usize >= CRED_TYPE.len() {
                break None;
            }

            let check = CRED_TYPE[self.index as usize];

            if self.cred.intersects(check) {
                break Some(CredentialType::from(check));
            }

            self.index += 1;
        }
    }
}
