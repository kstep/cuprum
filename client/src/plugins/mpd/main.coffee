angular.module 'ngenti.plugins.mpd', ['ui.bootstrap', 'ngResource', 'ngTouch']
    .config ['$routeProvider', ($route) ->
        $route.when '/mpd',
            templateUrl: 'plugins/mpd/main.html',
            controller: 'MPDController'
    ]

    .factory 'Queue', ['$resource', ($resource) ->
        $resource '/plugins/mpd/queue.json', {},
            query: { method: 'GET', isArray: true },
            remove: { method: 'DELETE', params: {id: '@id'} }
    ]
    .factory 'Stats', ['$resource', ($resource) ->
        $resource '/plugins/mpd/stats.json'
    ]
    .factory 'CurrentSong', ['$resource', ($resource) ->
        $resource '/plugins/mpd/current-song.json'
    ]
    .factory 'Outputs', ['$resource', ($resource) ->
        $resource '/plugins/mpd/outputs.json'
    ]
    .factory 'Playlists', ['$resource', ($resource) ->
        $resource '/plugins/mpd/playlists.json'
    ]
    .factory 'Player', ['$resource', ($resource) ->
        $resource '/plugins/mpd/player.json', {},
            get: { method: 'GET' },
            play: { method: 'GET',  params: {cmd: 'play'} }
            stop: { method: 'GET',  params: {cmd: 'stop'} }
            pause: { method: 'GET', params: {cmd: 'pause'} }
            next: { method: 'GET', params: {cmd: 'next'} }
            prev: { method: 'GET', params: {cmd: 'prev'} }
            seek: { method: 'GET', params: {cmd: 'seek'} }
            set: { method: 'GET', params: {cmd: 'set'} }
    ]

    .controller 'MPDController', ['$scope', 'Queue', 'CurrentSong', 'Outputs', 'Playlists', 'Player', '$window', ($scope, Queue, CurrentSong, Outputs, Playlists, Player, $window) ->
        $scope.$watch 'player.elapsed_time / player.total_time', (v) -> $scope.player.progress = v * 1000 if v

        $scope.queue = Queue.query()
        $scope.player = Player.get()
        # $scope.current_song = CurrentSong.get()
        $scope.playlists = Playlists.query()
        $scope.outputs = Outputs.query()

        $scope.library = [
            {artist: 'Alicia Keys', title: 'A Harlem Love Story (Fallin\' / A Woman\'s Worth)', genre: 'R&B/Soul', time: 10*60+4}
            {artist: 'Alicia Keys', title: 'Never Felt This Way (Interlude)', genre: 'Rock', time: 2*60+4}
            {artist: 'Alicia Keys', title: 'A Harlem Love Story (Fallin\' / A Woman\'s Worth)', genre: 'R&B/Soul', time: 10*60+4}
            {artist: 'Alicia Keys', title: 'Never Felt This Way (Interlude)', genre: 'Rock', time: 2*60+4}
            {artist: 'Alicia Keys', title: 'A Harlem Love Story (Fallin\' / A Woman\'s Worth)', genre: 'R&B/Soul', time: 10*60+4}
            {artist: 'Alicia Keys', title: 'Never Felt This Way (Interlude)', genre: 'Rock', time: 2*60+4}
            {artist: 'Alicia Keys', title: 'A Harlem Love Story (Fallin\' / A Woman\'s Worth)', genre: 'R&B/Soul', time: 10*60+4}
            {artist: 'Alicia Keys', title: 'Never Felt This Way (Interlude)', genre: 'Rock', time: 2*60+4}
            {artist: 'Alicia Keys', title: 'A Harlem Love Story (Fallin\' / A Woman\'s Worth)', genre: 'R&B/Soul', time: 10*60+4}
            {artist: 'Alicia Keys', title: 'Never Felt This Way (Interlude)', genre: 'Rock', time: 2*60+4}
        ]
    ]
