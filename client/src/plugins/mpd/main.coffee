angular.module 'ngenti.plugins.mpd', ['ui.bootstrap', 'ngResource', 'ngTouch']
    .config ['$routeProvider', ($route) ->
        $route.when '/mpd',
            templateUrl: 'plugins/mpd/main.html',
            controller: 'MPDController'
    ]

    .factory 'Queue', ['$resource', ($resource) ->
        $resource '/plugins/mpd/queue.json', {},
            query: { method: 'GET', isArray: true }
            load: { method: 'GET', isArray: true }
            save: { method: 'GET' }
            remove: { method: 'DELETE', params: {id: '@id'} }
    ]
    .factory 'Stats', ['$resource', ($resource) ->
        $resource '/plugins/mpd/stats.json'
    ]
    .factory 'CurrentSong', ['$resource', ($resource) ->
        $resource '/plugins/mpd/current-song.json'
    ]
    .factory 'Outputs', ['$resource', ($resource) ->
        $resource '/plugins/mpd/outputs.json', {},
            query: { method: 'GET', isArray: true },
            set: { method: 'GET', params: {cmd: 'set', id: '@id'} }
    ]
    .factory 'Playlists', ['$resource', ($resource) ->
        $resource '/plugins/mpd/playlists.json'
    ]
    .factory 'Player', ['$resource', ($resource) ->
        $resource '/plugins/mpd/player.json', {},
            get: { method: 'GET' },
            next: { method: 'GET', params: {cmd: 'next'} }
            prev: { method: 'GET', params: {cmd: 'prev'} }
            set: { method: 'GET', params: {cmd: 'set'} }
    ]

    .controller 'MPDController', ['$scope', 'Queue', 'CurrentSong', 'Outputs', 'Playlists', 'Player', '$modal', ($scope, Queue, CurrentSong, Outputs, Playlists, Player, $modal) ->
        $scope.$watch 'player.elapsed_time / player.total_time', (v) -> $scope.player.progress = v * 1000 if v

        $scope.queue = Queue.query()
        $scope.player = Player.get()
        # $scope.current_song = CurrentSong.get()
        $scope.playlists = Playlists.query()
        $scope.outputs = Outputs.query()

        $scope.volume_icon = (v) -> if v == 0 then "volume-off" else if v <= 50 then "volume-down" else "volume-up"

        $scope.remove = (song) ->
            $modal.open
                templateUrl: 'confirm-song-remove.html'
                keyboard: true
                controller: ['$scope', '$modalInstance', ($scope, $modal) ->
                    $scope.song = song
                    $scope.yes = -> $modal.close song
                    $scope.no = -> $modal.dismiss 'cancel'
                ]
            .result.then (song) -> song.$remove()

        $scope.load = (playlist) ->
            $scope.queue = Queue.load { path: playlist.path }

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
