function setup()
    print_rs("Lua loaded")
end

function process(delta)
    local player = get_player()
    if key_pressed("d") then
        print_rs(player:x())
        player:set_x(player:x() + player:speed() * delta)
    elseif key_pressed("a") then
        player:set_x(player:x() - player:speed() * delta)
    end

    if key_pressed("w") then
        player:set_y(player:y() - player:speed() * delta)
    elseif key_pressed("s") then
        player:set_y(player:y() + player:speed() * delta)
    end
end