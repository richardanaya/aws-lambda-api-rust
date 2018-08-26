resource "null_resource" "zip" {
	triggers { uuid = "${uuid()}" } 
	provisioner "local-exec" {
		# Create zip if zip doesn't exist or binary is newer than zip
		command = "test -e ../lambdas/rust-api.zip && test -z `find ../lambdas/ -name rust-api -newer ../lambdas/rust-api.zip` || zip -9 -j ../lambdas/rust-api.zip ../lambdas/rust-api"
	}
}