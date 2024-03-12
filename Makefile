build: #only builds the dev/debug version of the project in the targe/debug
	cargo build

run: #builds and runs the project in one command
	cargo run

check: #checks whether the code compiles successfully. Just like unit testing
	cargo check
	
watch: #live building and running the project as the code changes.
	redis-server --daemonize yes & nodemon --exec cargo run ./src/main.rs --signal SIGTERM

buildrelease: #builds the optimized version for production use and stores it in the target/release folder
	cargo build --release

runrelease: #runs the optimized version built for production.
	cargo run --release

checkrelease: #checks whether the release code compiles successfully. Just like unit testing
	cargo check --release

watchrelease: #live building and running the release project as the code changes.
	nodemon --exec cargo run --release ./src/main.rs --signal SIGTERM

db: #starts the mongodb server
	sudo mongod --dbpath ~/data/db --logpath ~/data/log/mongodb/mongo.log --fork

redis: #starts redis server
	redis-server --daemonize yes