.PHONY: clean clean-all install uninstall

target/release/eddit : src
	cargo build --release

install : target/release/eddit
	python make/pre_install.py
	cp target/release/eddit /usr/bin/com.encryptos.editor
	cp data/com.encryptos.editor.desktop /usr/share/applications/
	cp data/com.encryptos.editor.gschema.xml /usr/share/glib-2.0/schemas/
	cp data/com.encryptos.editor.appdata.xml /usr/share/metainfo/
	cp res/com.encryptos.editor.svg /usr/share/icons/hicolor/scalable/apps/
	cp data/styles/eddit-light.xml /usr/share/gtksourceview-3.0/styles/
	cp data/styles/eddit-dark.xml /usr/share/gtksourceview-3.0/styles/
	cp res/icons/day.svg /opt/com.encryptos.editor/icons/
	cp res/icons/night.svg /opt/com.encryptos.editor/icons/
	python make/post_install.py

uninstall :
	rm -f /usr/bin/com.encryptos.editor
	rm -f /usr/share/applications/com.encryptos.editor.desktop
	rm -f /usr/share/glib-2.0/schemas/com.encryptos.editor.gschema.xml
	rm -f /usr/share/metainfo/com.encryptos.editor.appdata.xml
	rm -f /usr/share/icons/hicolor/scalable/apps/com.encryptos.editor.svg
	rm -f /usr/share/gtksourceview-3.0/styles/eddit-light.xml
	rm -f /usr/share/gtksourceview-3.0/styles/eddit-dark.xml
	rm -rf /opt/com.encryptos.editor

clean-all : clean
	cargo clean

clean :
	true