provider "aws" {
  shared_credentials_file = "$HOME/.aws/credentials"
  profile                 = "earth"
  region                  = var.aws_region
}

