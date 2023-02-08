resource "aws_dynamodb_table" "url_shortener_table" {
  hash_key       = "short_url"
  name           = "Url_Shortener"
  billing_mode   = "PAY_PER_REQUEST"
  read_capacity  = 0
  write_capacity = 0

  attribute {
    name = "short_url"
    type = "S"
  }
}
