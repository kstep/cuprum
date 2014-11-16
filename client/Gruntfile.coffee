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
                dest: 'build/libs.js'
                cssDest: 'build/libs.css'

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

        watch:
            options:
                cwd: 'src'
                interrupt: true
            coffee:
                files: ['**/*.coffee']
                tasks: ['coffee:compile', 'uglify:build']
            less:
                files: ['**/*.less']
                tasks: ['less:compile', 'cssmin:build']
            jade:
                files: ['**/*.jade']
                tasks: ['jade:compile']

        clean: [
            "build"
            "dest"
        ]

        parallel:
            dev:
                options:
                    stream: true
                tasks: [
                    { grunt: true, args: ['watch'] }
                    { grunt: true, args: ['connect'] }
                ]
            compile:
                tasks: [
                    { grunt: true, args: ['bower:install', 'bower_concat:all'] }
                    { grunt: true, args: ['coffee:compile'] }
                    { grunt: true, args: ['less:compile'] }
                    { grunt: true, args: ['jade:compile'] }
                ]
            build:
                tasks: [
                    { grunt: true, args: ['uglify:build'] }
                    { grunt: true, args: ['cssmin:build'] }
                    { grunt: true, args: ['copy:fonts'] }
                ]

    grunt.loadNpmTasks 'grunt-contrib-uglify'
    grunt.loadNpmTasks 'grunt-contrib-coffee'
    grunt.loadNpmTasks 'grunt-contrib-cssmin'
    grunt.loadNpmTasks 'grunt-contrib-jade'
    grunt.loadNpmTasks 'grunt-contrib-less'
    grunt.loadNpmTasks 'grunt-contrib-copy'
    grunt.loadNpmTasks 'grunt-contrib-connect'
    grunt.loadNpmTasks 'grunt-contrib-watch'
    grunt.loadNpmTasks 'grunt-contrib-clean'
    grunt.loadNpmTasks 'grunt-bower-task'
    grunt.loadNpmTasks 'grunt-bower-concat'
    grunt.loadNpmTasks 'grunt-parallel'

    grunt.registerTask 'default', [
        'parallel:compile'
        'parallel:build'
    ]
