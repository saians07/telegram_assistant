build-rpi:
	CARGO_BUILD_TARGET=aarch64-unknown-linux-gnu cargo build --release

upload-to-server:
	rsync target/aarch64-unknown-linux-gnu/release/swan saians07@192.168.1.10:~/projects/telegram_assistant/swan && \
	rsync .env saians07@192.168.1.10:~/projects/telegram_assistant/.env
