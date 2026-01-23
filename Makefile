.PHONY: build clean serve dev test help

# 默认目标
help:
	@echo "使用方法:"
	@echo "  make build   - 构建静态网站到 docs/ (用于 GitHub Pages)"
	@echo "  make clean   - 清理构建产物"
	@echo "  make serve   - 本地预览网站 (端口 8000)"
	@echo "  make dev     - 为本地开发构建并预览"
	@echo "  make test    - 运行测试"
	@echo "  make release - 编译发布版本"

# 构建静态网站 (用于 GitHub Pages)
build:
	@echo "🔨 构建静态网站..."
	@cargo run --manifest-path generator/Cargo.toml -- build
	@echo "✅ 构建完成! 输出目录: docs/"

# 构建用于本地开发的版本 (base_path 为空)
dev-build:
	@echo "🔨 构建本地开发版本..."
	@cargo run --manifest-path generator/Cargo.toml -- build --base-path ""
	@echo "✅ 构建完成! 输出目录: docs/"

# 清理构建产物
clean:
	@echo "🧹 清理..."
	@rm -rf docs/
	@cargo clean --manifest-path generator/Cargo.toml
	@echo "✅ 清理完成"

# 本地预览
serve:
	@echo "🌐 启动本地服务器: http://localhost:8000/"
	@cd docs && python3 -m http.server 8000

# 构建并预览 (本地开发)
dev: dev-build serve

# 运行测试
test:
	@echo "🧪 运行测试..."
	@cargo test --manifest-path generator/Cargo.toml
	@echo "✅ 测试完成"

# 编译发布版本
release:
	@echo "📦 编译发布版本..."
	@cargo build --manifest-path generator/Cargo.toml --release
	@echo "✅ 可执行文件: generator/target/release/xavierxcn-generator"
