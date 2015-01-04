angular.module 'ngenti', ['ng', 'ngRoute', 'ngenti.plugins.mpd', 'ngenti.plugins.dnsmasq']
    .filter 'time', [->
        d2 = (v) -> if (v < 10)
                '0' + v
            else
                v

        (value) -> "#{Math.floor(value / 60)}:#{d2(Math.floor(value % 60))}"
    ]
    .controller 'NavigationController', ['$scope', '$location', ($scope, $location) ->
        $scope.$location = $location
        $scope.plugins = [
            {module: 'ngenti.plugins.dashboard', route: '/', icon: 'dashboard', name: 'Dashboard', title: 'Dashboard'},
            {module: 'ngenti.plugins.mpd', route: '/mpd', icon: 'music', name: 'MPD', title: 'Music Player Daemon'},
            {module: 'ngenti.plugins.transmission', icon: 'download-alt', route: '/transmission', name: 'Transmission', title: 'Transmission Torrent Daemon'},
            {module: 'ngenti.plugins.nginx', icon: 'cloud', route: '/nginx', name: 'Nginx', title: 'Nginx HTTP Server'},
            {module: 'ngenti.plugins.dnsmasq', icon: 'globe', route: '/dnsmasq', name: 'Dnsmasq', title: 'Dnsmasq DNS Server'}
        ]
    ]
    .controller 'ContentController', ['$scope', ($scope) ->
        $scope.floor = Math.floor
        $scope.ceil = Math.ceil
        $scope.hasField = (field) -> (item) -> !!item[field]

    ]
