TABLE_LIST = \
	users, \
	telegram_users, \
	telegram_chat_history, \
	telegram_guest, \
	telegram_guest_chat_history, \
    telegram_chat_queue

TABLES = $(subst $(grid) ,, $(strip $(TABLE_LIST)))

db-migrate-up:
	sea-orm-cli migrate up
	sea-orm-cli generate entity -o src/entities --tables $(TABLES) --with-serde both

build-rpi:
	CARGO_BUILD_TARGET=aarch64-unknown-linux-gnu cargo build --release

upload-to-server:
	rsync -avz --progress target/aarch64-unknown-linux-gnu/release/swan saians07@192.168.1.10:~/projects/telegram_assistant/swan && \
	rsync -avz --progress .env saians07@192.168.1.10:~/projects/telegram_assistant/.env
