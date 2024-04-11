local JSON = require('./lua/json');
local modeTable = require('./lua/modes');

-- Process the modes
do
    for k,mode in pairs(modeTable) do
        if not mode.unlock then
            modeTable[k].unlock = {};
        end
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