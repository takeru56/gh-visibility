# gh-visibility
Command line interface to change visibility of GitHub repositories.

You can switch public/private of your repository easily.

## Usage
### Authentication
Please prepare an [personal access token](https://docs.github.com/ja/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token) to access GitHub repository.In the scope selection, choose **repo**.

- Set token as an environment variable in shell
```
export GITHUB_AUTH_TOKEN="your_token"
```

OR

- Set token when executing commands
```
 GITHUB_AUTH_TOKEN="your_token" COMMAND
```
### Help
```
Usage: gh-visibility COMMAND USER_NAME

Command line interface to change visibility of GitHub repositories.

Commands:
  repos      list repositories with visibility status
  change     change visibility of the repository

```

### List your repositories with visibility status
```
$ gh-visibility repos USER_NAME
```

ex.
```
$ gh-visibility repos takeru56
repo_name              visibility description
-=====================-==========-==================================================-
|dotfiles              |PUBLIC    |Manage my dotofiles                               
|CryptoViewer          |PUBLIC    |Chrome's extension to check the rate of crypto currency
|takeru56.github.io    |PRIVATE   |                                                  
|draiw.io              |PUBLIC    |Stock my diagrams at draw.io                      
|App                   |PUBLIC    |Control PWM on Raspberry Pi                       
|tanaken.me            |PRIVATE   |Personal Blog site Powered by Hugo                
|gh-visibility         |PUBLIC    |Command line interface to change visibility of GitHub repo
...
```

### Change visibility of the repository
#### single repo
```
$ gh-visibility change USER_NAME REPO_NAME:[public|private]
```
ex.
- switch to private repo
```
$ gh-visibility change takeru56 gh-visibility:private
"gh-visibility" is now private: true
```

- switch to public repo
```
$ gh-visibility change takeru56 gh-visibility:public
"gh-visibility" is now private: false
```

#### multiple
```
$ gh-visibility change USER_NAME REPO_NAME:[public|private] REPO_NAME:[public|private]...
```

## Install
### Docker
- install
```
$ git clone git@github.com:takeru56/gh-visibility.git
$ cd gh-visibility
$ docker build -t gh-visibility .
```

- run
```
$ docker run -e GITHUB_AUTH_TOKEN="your_token" gh-visibility repos USER_NAME
```

### From source
todo

## License
MIT

