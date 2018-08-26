resource "null_resource" "zip" {
	triggers { uuid = "${uuid()}" } 
	provisioner "local-exec" {
		# Create zip if binary is newer, otherwise do nothing
		command = "test `find ../lambdas/ -name rust-api -newer ../lambdas/rust-api.zip` && zip -9 ../lambdas/rust-api.zip ../lambdas/rust-api || true"
	}
}