# Workflow:
# src -> build -> dest
# src: less & coffee files
# build: compiled css & js files
# dest: resulting bundled and minified files

module.exports = (grunt) ->

    grunt.initConfig
        pkg: grunt.file.readJSON "package.json"

        bower:
            install: {}

        bower_concat:
            all:
                dest: 'build/bower.js'
                cssDest: 'build/bower.css'

        copy:
            fonts:
                files: [
                    {expand: true, cwd: 'bower_components/bootstrap/', src: ['fonts/*.eot', 'fonts/*.woff', 'fonts/*.ttf', 'fonts/*.svg'], dest: 'dest/'}
                ]

        coffee:
            compile:
                options:
                    bare: true
                    sourceMap: true
                expand: true
                flatten: false
                cwd: 'src'
                src: ['**/*.coffee']
                dest: 'build'
                ext: '.js'

        jade:
            compile:
                expand: true
                cwd: 'src'
                src: ['**/*.jade']
                dest: 'dest'
                ext: '.html'

        less:
            compile:
                expand: true
                cwd: 'src'
                src: ['**/*.less']
                dest: 'build'
                ext: '.css'

        cssmin:
            build:
                expand: true
                cwd: 'build'
                src: ['*.css', '!*.min.css']
                dest: 'dest'
                ext: '.min.css'

        uglify:
            options:
                banner: '/*! <%= pkg.name %> <%= grunt.template.today("yyyy-mm-dd") %> */\n'
                sourceMap: true
            build:
                files: [
                    expand: true
                    cwd: 'build'
                    src: '**/*.js'
                    dest: 'dest'
                    ext: '.min.js'
                ]

        connect:
            server:
                options:
                    port: 8080
                    base: 'dest'
                    keepalive: true

    grunt.loadNpmTasks 'grunt-contrib-uglify'
    grunt.loadNpmTasks 'grunt-contrib-coffee'
    grunt.loadNpmTasks 'grunt-contrib-cssmin'
    grunt.loadNpmTasks 'grunt-contrib-jade'
    grunt.loadNpmTasks 'grunt-contrib-less'
    grunt.loadNpmTasks 'grunt-contrib-copy'
    grunt.loadNpmTasks 'grunt-contrib-connect'
    grunt.loadNpmTasks 'grunt-bower-task'
    grunt.loadNpmTasks 'grunt-bower-concat'

    grunt.registerTask 'default', [
        'bower:install'
        'bower_concat:all'
        'copy:fonts'
        'coffee:compile'
        'less:compile'
        'jade:compile'
        'uglify:build'
        'cssmin:build'
    ]
