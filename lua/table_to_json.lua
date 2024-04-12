local JSON = require('./lua/json');
local modeTable = require('./lua/modes');

-- Process the modes
do
    for k,mode in pairs(modeTable) do
        if not mode.unlock then
            modeTable[k].unlock = {};
        end
        modeTable[k].source = string.format(
            "https://github.com/26F-Studio/Techmino/tree/main/parts/modes/%s.lua",
            mode.name
        );
    end
    local newModeTable = {};
    for _,mode in pairs(modeTable) do
        newModeTable[mode.name] = mode;
    end
    modeTable = newModeTable;
end

-- Convert to JSON
local modeJSON = JSON.encode(modeTable);
print(modeJSON);