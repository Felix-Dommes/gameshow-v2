redeploy: clean deploy

deploy: build
	mv ./gameshow/dist/* ../static/

clean:
	rm -rf ../static/*

build:
	cd ./gameshow && \
	npm run build
