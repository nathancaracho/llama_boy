frontend:
	cd front_end
	trunk serve --port 3000

backend:
	cargo run -p back_end

all:
	bash -c "just backend & just frontend & wait"
