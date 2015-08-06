#!/bin/bash

# This is my recursive version of the @micw/@chuxau script
# https://gist.github.com/chuxau/6bc42f0f271704cd4e91

UPDATES_URL="http://updates.jenkins-ci.org/download/plugins/"

if [ $# -lt 2 ]; then
    echo "USAGE: $0 plugin-list-file destination-directory"
    exit 1
fi

plugin_list=$1
plugin_dir=$2

mkdir -p $plugin_dir

installPlugin() {
    if [ -f ${plugin_dir}/${1}.hpi -o -f ${plugin_dir}/${1}.jpi ]; then
	if [ "$2" == "1" ]; then
	    return 1
	fi
	echo "Skipped: $1 (already installed)"
    else
	echo "Installing: $1"
	wget -q ${UPDATES_URL}/${1}/${2}/${1}.hpi -O ${plugin_dir}/${1}.hpi

	deps=$(unzip -p  ${plugin_dir}/${1}.hpi META-INF/MANIFEST.MF \
		      | tr -d '\r' \
		      | sed -e ':a;N;$!ba;s/\n //g' \
		      | grep -e 'Plugin-Dependencies' \
		      | awk '{print $2}' \
		      | tr "," "\n" \
		      | grep -v 'resolution:=optional')

	if [[ -n $deps ]]; then
	    echo $deps \
		| tr ' ' '\n' \
		| while IFS=":" read plugin version; do
		      installPlugin $plugin $version
		  done
	    changed=1
	fi
    fi
    return 0
}

while IFS=":" read plugin version; do
    if [[ $plugin =~ ^# ]]; then
	continue
    fi
    installPlugin $plugin $version
done < $plugin_list

echo "Plugins installed!"
