#!/bin/bash
###
 # @Author: timochan
 # @Date: 2023-02-03 19:45:22
 # @LastEditors: timochan
 # @LastEditTime: 2023-02-03 20:52:49
 # @FilePath: /catwrt-update/catwrt_update.sh
### 
get_remote_version() {
    version_remote=`curl https://api.miaoer.xyz/api/v2/snippets/catwrt/check_update | jq -r '.version'`
    if [ $? -ne 0 ] || [ -z $version_remote ]; then
        echo "Remote version get failed, please check your network!"
        exit 1
    fi
   
    hash_remote=`curl https://api.miaoer.xyz/api/v2/snippets/catwrt/check_update | jq -r '.hash'`
    if [ $? -ne 0 ] || [ -z $hash_remote ]; then
        echo "Remote hash get failed, please check your network!"
        exit 1
    fi
}

get_local_version(){
    version_local=`cat /etc/catwrt_release | grep 'version' | cut -d '=' -f 2`
    if [ $? -ne 0 ]; then
        echo "Local version get failed, please check your /etc/catwrt_release!"
        exit 1
    fi
    hash_local=`cat /etc/catwrt_release | grep 'hash' | cut -d '=' -f 2`
    if [ $? -ne 0 ]; then
        echo "Local hash get failed, please check your /etc/catwrt_release!"
        exit 1
    fi

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
    get_remote_version
    get_local_version
    contrast_version
    print_version
}
main