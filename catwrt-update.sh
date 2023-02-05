#!/bin/bash
###
 # @Author: timochan
 # @Date: 2023-02-03 19:45:22
 # @LastEditors: timochan
 # @LastEditTime: 2023-02-06 00:19:13
 # @FilePath: /catwrt-update/catwrt-update.sh
### 
remote_error() {
    echo "Remote $1 get failed, please check your network!"
    exit 1
}
local_error() {
    echo "Local $1 get failed, please check your /etc/catwrt-release!"
    exit 1
}
get_remote_version(){
    arch_self=$1
    version_remote=`curl https://api.miaoer.xyz/api/v2/snippets/catwrt/check_update | jq -r '.version'`
    if [ $? -ne 0 ] || [ -z $version_remote ]; then
        remote_error "version"
        exit 1
    fi
    if [ $arch_self == "x86_64" ]; then
        hash_remote=`curl https://api.miaoer.xyz/api/v2/snippets/catwrt/check_update | jq -r '.hash_amd64'`
    elif [ $arch_self == "aarch64" ]; then
        hash_remote=`curl https://api.miaoer.xyz/api/v2/snippets/catwrt/check_update | jq -r '.hash_arm'`
    elif [ $arch_self == "mips" ]; then
        hash_remote=`curl https://api.miaoer.xyz/api/v2/snippets/catwrt/check_update | jq -r '.hash_wireless'`
    else 
        echo "Your system is not supported!"
        exit 1
    fi

    if [ $? -ne 0 ] || [ -z $hash_remote ]; then
        remote_error "hash"
        exit 1
    fi
}
get_arch_and_remote_version() {
    arch=`uname -m`
    if [ $? -ne 0 ] || [ -z $arch ]; then
        echo "Arch get failed, please check your system!"
        exit 1
    fi

    get_remote_version $arch

}
get_local_version(){

    if [ ! -f /etc/catwrt-release ]; then
        local_error "version file"
        exit 1
    fi
    version_local=`cat /etc/catwrt-release | grep 'version' | cut -d '=' -f 2`
    hash_local=`cat /etc/catwrt-release | grep 'hash' | cut -d '=' -f 2`


}
contrast_version(){
    if [ $version_remote == $version_local ] && [ $hash_remote == $hash_local ]; then
        echo "Your CatWrt is up to date!"
    else
        echo "Your CatWrt is not up to date, You should upgrade it!"
        echo "You can visit https://www.miaoer.xyz/posts/network/catwrt to get more information!"
    fi
}
print_version(){
        echo "Local  Version : $version_local"
        echo "Remote Version : $version_remote"
        echo "Local  Hash : $hash_local"
        echo "Remote Hash : $hash_remote"
}
main(){
    get_arch_and_remote_version
    get_local_version
    contrast_version
    print_version
}
main